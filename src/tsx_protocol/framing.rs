use std::io::{ErrorKind, Read, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{GuiError, GuiResult};

pub const TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES: u32 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsxFrameLimitsV1 {
    maximum_payload_bytes: u32,
}

impl TsxFrameLimitsV1 {
    pub fn new(maximum_payload_bytes: u32) -> GuiResult<Self> {
        if maximum_payload_bytes == 0 {
            return Err(GuiError::host("TSX protocol frame limit must be non-zero"));
        }
        if maximum_payload_bytes > TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES {
            return Err(GuiError::host(format!(
                "TSX protocol frame limit {maximum_payload_bytes} exceeds the version-1 hard limit {TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES}"
            )));
        }
        Ok(Self {
            maximum_payload_bytes,
        })
    }

    pub const fn maximum_payload_bytes(self) -> u32 {
        self.maximum_payload_bytes
    }
}

impl Default for TsxFrameLimitsV1 {
    fn default() -> Self {
        Self {
            maximum_payload_bytes: TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
        }
    }
}

pub fn encode_tsx_json_frame_v1<T>(message: &T, limits: TsxFrameLimitsV1) -> GuiResult<Vec<u8>>
where
    T: Serialize,
{
    let payload = serde_json::to_vec(message).map_err(|error| {
        GuiError::host(format!(
            "TSX protocol message could not be serialized as JSON: {error}"
        ))
    })?;
    validate_payload_length(payload.len(), limits)?;
    let length = u32::try_from(payload.len())
        .map_err(|_| GuiError::host("TSX protocol payload length does not fit in u32"))?;
    let mut frame = Vec::with_capacity(4 + payload.len());
    frame.extend_from_slice(&length.to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

pub fn decode_tsx_json_payload_v1<T>(payload: &[u8], limits: TsxFrameLimitsV1) -> GuiResult<T>
where
    T: DeserializeOwned,
{
    validate_payload_length(payload.len(), limits)?;
    let json = std::str::from_utf8(payload).map_err(|error| {
        GuiError::host(format!("TSX protocol payload is not valid UTF-8: {error}"))
    })?;
    serde_json::from_str(json).map_err(|error| {
        GuiError::host(format!(
            "TSX protocol payload is not a valid strict message: {error}"
        ))
    })
}

pub fn write_tsx_json_frame_v1<W, T>(
    writer: &mut W,
    message: &T,
    limits: TsxFrameLimitsV1,
) -> GuiResult<()>
where
    W: Write,
    T: Serialize,
{
    let frame = encode_tsx_json_frame_v1(message, limits)?;
    writer
        .write_all(&frame)
        .map_err(|error| GuiError::host(format!("could not write TSX protocol frame: {error}")))?;
    writer
        .flush()
        .map_err(|error| GuiError::host(format!("could not flush TSX protocol frame: {error}")))
}

pub fn read_tsx_json_frame_v1<R, T>(
    reader: &mut R,
    limits: TsxFrameLimitsV1,
) -> GuiResult<Option<T>>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut header = [0_u8; 4];
    let header_bytes = read_exact_or_eof(reader, &mut header, "length prefix")?;
    if header_bytes == 0 {
        return Ok(None);
    }
    let length = u32::from_le_bytes(header);
    validate_declared_length(length, limits)?;
    let mut payload = vec![0_u8; length as usize];
    let payload_bytes = read_exact_or_eof(reader, &mut payload, "JSON payload")?;
    if payload_bytes != payload.len() {
        return Err(GuiError::host(format!(
            "TSX protocol frame ended after {payload_bytes} of {} payload bytes",
            payload.len()
        )));
    }
    decode_tsx_json_payload_v1(&payload, limits).map(Some)
}

#[derive(Debug)]
enum TsxFrameDecodeStateV1 {
    Length { bytes: [u8; 4], filled: usize },
    Payload { expected: usize, bytes: Vec<u8> },
}

impl Default for TsxFrameDecodeStateV1 {
    fn default() -> Self {
        Self::Length {
            bytes: [0; 4],
            filled: 0,
        }
    }
}

/// Incremental decoder for a byte stream carrying consecutive TSX messages.
///
/// The decoder validates the declared size before allocating payload storage.
/// Any framing or JSON failure poisons the decoder because stream boundaries
/// are no longer trustworthy after a protocol violation.
#[derive(Debug)]
pub struct TsxJsonFrameDecoderV1 {
    limits: TsxFrameLimitsV1,
    state: TsxFrameDecodeStateV1,
    poisoned: bool,
}

impl TsxJsonFrameDecoderV1 {
    pub fn new(limits: TsxFrameLimitsV1) -> Self {
        Self {
            limits,
            state: TsxFrameDecodeStateV1::default(),
            poisoned: false,
        }
    }

    pub fn limits(&self) -> TsxFrameLimitsV1 {
        self.limits
    }

    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    pub fn push<T>(&mut self, mut chunk: &[u8]) -> GuiResult<Vec<T>>
    where
        T: DeserializeOwned,
    {
        if self.poisoned {
            return Err(GuiError::host(
                "TSX protocol frame decoder is poisoned after an earlier failure",
            ));
        }

        let mut messages = Vec::new();
        while !chunk.is_empty() {
            match &mut self.state {
                TsxFrameDecodeStateV1::Length { bytes, filled } => {
                    let count = (4 - *filled).min(chunk.len());
                    bytes[*filled..*filled + count].copy_from_slice(&chunk[..count]);
                    *filled += count;
                    chunk = &chunk[count..];
                    if *filled == 4 {
                        let length = u32::from_le_bytes(*bytes);
                        if let Err(error) = validate_declared_length(length, self.limits) {
                            self.poisoned = true;
                            return Err(error);
                        }
                        self.state = TsxFrameDecodeStateV1::Payload {
                            expected: length as usize,
                            bytes: Vec::with_capacity(length as usize),
                        };
                    }
                }
                TsxFrameDecodeStateV1::Payload { expected, bytes } => {
                    let count = (*expected - bytes.len()).min(chunk.len());
                    bytes.extend_from_slice(&chunk[..count]);
                    chunk = &chunk[count..];
                    if bytes.len() == *expected {
                        let payload = std::mem::take(bytes);
                        self.state = TsxFrameDecodeStateV1::default();
                        match decode_tsx_json_payload_v1(&payload, self.limits) {
                            Ok(message) => messages.push(message),
                            Err(error) => {
                                self.poisoned = true;
                                return Err(error);
                            }
                        }
                    }
                }
            }
        }
        Ok(messages)
    }

    pub fn finish(&self) -> GuiResult<()> {
        if self.poisoned {
            return Err(GuiError::host(
                "TSX protocol frame decoder is poisoned after an earlier failure",
            ));
        }
        match &self.state {
            TsxFrameDecodeStateV1::Length { filled: 0, .. } => Ok(()),
            TsxFrameDecodeStateV1::Length { filled, .. } => Err(GuiError::host(format!(
                "TSX protocol stream ended after {filled} of 4 length-prefix bytes"
            ))),
            TsxFrameDecodeStateV1::Payload { expected, bytes } => Err(GuiError::host(format!(
                "TSX protocol stream ended after {} of {expected} payload bytes",
                bytes.len()
            ))),
        }
    }
}

impl Default for TsxJsonFrameDecoderV1 {
    fn default() -> Self {
        Self::new(TsxFrameLimitsV1::default())
    }
}

fn validate_payload_length(length: usize, limits: TsxFrameLimitsV1) -> GuiResult<()> {
    let length = u32::try_from(length)
        .map_err(|_| GuiError::host("TSX protocol payload length does not fit in u32"))?;
    validate_declared_length(length, limits)
}

fn validate_declared_length(length: u32, limits: TsxFrameLimitsV1) -> GuiResult<()> {
    if length == 0 {
        return Err(GuiError::host(
            "TSX protocol frames cannot have an empty payload",
        ));
    }
    if length > limits.maximum_payload_bytes {
        return Err(GuiError::host(format!(
            "TSX protocol frame declares {length} payload bytes, exceeding the negotiated {}-byte limit",
            limits.maximum_payload_bytes
        )));
    }
    Ok(())
}

fn read_exact_or_eof<R: Read>(reader: &mut R, buffer: &mut [u8], part: &str) -> GuiResult<usize> {
    let mut filled = 0;
    while filled < buffer.len() {
        match reader.read(&mut buffer[filled..]) {
            Ok(0) => break,
            Ok(count) => filled += count,
            Err(error) if error.kind() == ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(GuiError::host(format!(
                    "could not read TSX protocol {part}: {error}"
                )))
            }
        }
    }
    if filled != 0 && filled != buffer.len() && buffer.len() == 4 {
        return Err(GuiError::host(format!(
            "TSX protocol frame ended after {filled} of 4 length-prefix bytes"
        )));
    }
    Ok(filled)
}

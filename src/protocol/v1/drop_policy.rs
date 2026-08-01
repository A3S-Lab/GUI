use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolDropOperationV1 {
    Copy,
    Move,
    Link,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolDropPositionV1 {
    Before,
    On,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum ProtocolCollectionDropTargetV1 {
    Root,
    Item {
        key: String,
        #[serde(rename = "dropPosition")]
        drop_position: ProtocolDropPositionV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum ProtocolDropPolicyTargetV1 {
    Generic {
        id: String,
    },
    Collection {
        id: String,
        target: ProtocolCollectionDropTargetV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum ProtocolDropPolicyRequestV1 {
    ShouldAcceptItemDrop {
        target: ProtocolDropPolicyTargetV1,
        types: Vec<String>,
    },
    GetDropOperation {
        target: ProtocolDropPolicyTargetV1,
        types: Vec<String>,
        allowed_operations: Vec<ProtocolDropOperationV1>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ProtocolDropPolicyDecisionV1 {
    AcceptItemDrop { accepted: bool },
    DropOperation { operation: ProtocolDropOperationV1 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolDropPolicyQueryPayloadV1 {
    pub query_sequence: u64,
    pub policy_id: String,
    pub request: ProtocolDropPolicyRequestV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolDropPolicyResponsePayloadV1 {
    pub query_sequence: u64,
    pub decision: ProtocolDropPolicyDecisionV1,
}

pub type ProtocolDropPolicyQueryV1 = ProtocolEnvelopeV1<ProtocolDropPolicyQueryPayloadV1>;
pub type ProtocolDropPolicyResponseV1 = ProtocolEnvelopeV1<ProtocolDropPolicyResponsePayloadV1>;

#[cfg(feature = "platform-runtime")]
mod self_drawn {
    use crate::platform_runtime::{
        SelfDrawnCollectionDropTarget, SelfDrawnDropOperation, SelfDrawnDropPolicyDecision,
        SelfDrawnDropPolicyQuery, SelfDrawnDropPolicyRequest, SelfDrawnDropPolicyResolution,
        SelfDrawnDropPolicyResolver, SelfDrawnDropPolicyResponse, SelfDrawnDropPolicyTarget,
        SelfDrawnDropPosition,
    };

    use super::*;

    impl From<SelfDrawnDropOperation> for ProtocolDropOperationV1 {
        fn from(value: SelfDrawnDropOperation) -> Self {
            match value {
                SelfDrawnDropOperation::Copy => Self::Copy,
                SelfDrawnDropOperation::Move => Self::Move,
                SelfDrawnDropOperation::Link => Self::Link,
                SelfDrawnDropOperation::Cancel => Self::Cancel,
            }
        }
    }

    impl From<ProtocolDropOperationV1> for SelfDrawnDropOperation {
        fn from(value: ProtocolDropOperationV1) -> Self {
            match value {
                ProtocolDropOperationV1::Copy => Self::Copy,
                ProtocolDropOperationV1::Move => Self::Move,
                ProtocolDropOperationV1::Link => Self::Link,
                ProtocolDropOperationV1::Cancel => Self::Cancel,
            }
        }
    }

    impl From<SelfDrawnDropPosition> for ProtocolDropPositionV1 {
        fn from(value: SelfDrawnDropPosition) -> Self {
            match value {
                SelfDrawnDropPosition::Before => Self::Before,
                SelfDrawnDropPosition::On => Self::On,
                SelfDrawnDropPosition::After => Self::After,
            }
        }
    }

    impl From<&SelfDrawnCollectionDropTarget> for ProtocolCollectionDropTargetV1 {
        fn from(value: &SelfDrawnCollectionDropTarget) -> Self {
            match value {
                SelfDrawnCollectionDropTarget::Root => Self::Root,
                SelfDrawnCollectionDropTarget::Item { key, drop_position } => Self::Item {
                    key: key.clone(),
                    drop_position: (*drop_position).into(),
                },
            }
        }
    }

    impl From<&SelfDrawnDropPolicyTarget> for ProtocolDropPolicyTargetV1 {
        fn from(value: &SelfDrawnDropPolicyTarget) -> Self {
            match value {
                SelfDrawnDropPolicyTarget::Generic { id } => Self::Generic {
                    id: id.as_str().to_string(),
                },
                SelfDrawnDropPolicyTarget::Collection { id, target } => Self::Collection {
                    id: id.as_str().to_string(),
                    target: target.into(),
                },
            }
        }
    }

    impl From<&SelfDrawnDropPolicyRequest> for ProtocolDropPolicyRequestV1 {
        fn from(value: &SelfDrawnDropPolicyRequest) -> Self {
            match value {
                SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { target, types } => {
                    Self::ShouldAcceptItemDrop {
                        target: target.into(),
                        types: types.clone(),
                    }
                }
                SelfDrawnDropPolicyRequest::GetDropOperation {
                    target,
                    types,
                    allowed_operations,
                } => Self::GetDropOperation {
                    target: target.into(),
                    types: types.clone(),
                    allowed_operations: allowed_operations
                        .iter()
                        .copied()
                        .map(Into::into)
                        .collect(),
                },
            }
        }
    }

    impl From<SelfDrawnDropPolicyDecision> for ProtocolDropPolicyDecisionV1 {
        fn from(value: SelfDrawnDropPolicyDecision) -> Self {
            match value {
                SelfDrawnDropPolicyDecision::AcceptItemDrop { accepted } => {
                    Self::AcceptItemDrop { accepted }
                }
                SelfDrawnDropPolicyDecision::DropOperation { operation } => Self::DropOperation {
                    operation: operation.into(),
                },
            }
        }
    }

    impl From<ProtocolDropPolicyDecisionV1> for SelfDrawnDropPolicyDecision {
        fn from(value: ProtocolDropPolicyDecisionV1) -> Self {
            match value {
                ProtocolDropPolicyDecisionV1::AcceptItemDrop { accepted } => {
                    Self::AcceptItemDrop { accepted }
                }
                ProtocolDropPolicyDecisionV1::DropOperation { operation } => Self::DropOperation {
                    operation: operation.into(),
                },
            }
        }
    }

    /// Wraps one portable runtime query in the strict version-1 session
    /// envelope consumed by a TSX policy process.
    pub fn protocol_drop_policy_query_v1(
        session_id: impl Into<String>,
        query: &SelfDrawnDropPolicyQuery,
    ) -> ProtocolDropPolicyQueryV1 {
        ProtocolEnvelopeV1::new(
            ProtocolMetadataV1::event(session_id, query.frame_revision.get(), query.event_sequence),
            ProtocolDropPolicyQueryPayloadV1 {
                query_sequence: query.query_sequence,
                policy_id: query.policy_id.clone(),
                request: (&query.request).into(),
            },
        )
    }

    /// Validates a process response against its active session and exact
    /// runtime query before returning a response the portable resolver accepts.
    pub fn self_drawn_drop_policy_response_from_v1(
        response: ProtocolDropPolicyResponseV1,
        expected_session_id: &str,
        query: &SelfDrawnDropPolicyQuery,
    ) -> GuiResult<SelfDrawnDropPolicyResponse> {
        if response.metadata.protocol_version != NATIVE_PROTOCOL_VERSION_V1 {
            return Err(GuiError::host(format!(
                "drop policy response protocol version {} does not match {}",
                response.metadata.protocol_version, NATIVE_PROTOCOL_VERSION_V1
            )));
        }
        if response.metadata.session_id != expected_session_id {
            return Err(GuiError::host(
                "drop policy response session does not match the active session",
            ));
        }
        if response.metadata.render_revision != query.frame_revision.get()
            || response.metadata.event_sequence != Some(query.event_sequence)
            || response.payload.query_sequence != query.query_sequence
        {
            return Err(GuiError::host(
                "drop policy response does not match the active frame, event, and query",
            ));
        }
        Ok(SelfDrawnDropPolicyResponse {
            frame_revision: query.frame_revision,
            event_sequence: query.event_sequence,
            query_sequence: query.query_sequence,
            decision: response.payload.decision.into(),
        })
    }

    /// Result of one bounded query/response exchange with the TSX policy
    /// process. The transport owns the deadline; the resolver always maps a
    /// non-response outcome to the corresponding fail-closed runtime result.
    #[derive(Debug, Clone, PartialEq)]
    pub enum ProtocolDropPolicyExchangeResultV1 {
        Response(ProtocolDropPolicyResponseV1),
        TimedOut,
        Unavailable,
        Failed,
    }

    pub trait ProtocolDropPolicyExchangeV1 {
        fn exchange(
            &mut self,
            query: ProtocolDropPolicyQueryV1,
        ) -> ProtocolDropPolicyExchangeResultV1;
    }

    impl<F> ProtocolDropPolicyExchangeV1 for F
    where
        F: FnMut(ProtocolDropPolicyQueryV1) -> ProtocolDropPolicyExchangeResultV1,
    {
        fn exchange(
            &mut self,
            query: ProtocolDropPolicyQueryV1,
        ) -> ProtocolDropPolicyExchangeResultV1 {
            self(query)
        }
    }

    /// Concrete adapter between the portable synchronous resolver and the
    /// strict version-1 process protocol.
    pub struct ProtocolDropPolicyResolverV1<E> {
        session_id: String,
        exchange: E,
    }

    impl<E> ProtocolDropPolicyResolverV1<E> {
        pub fn new(session_id: impl Into<String>, exchange: E) -> GuiResult<Self> {
            let session_id = session_id.into();
            if session_id.trim().is_empty() {
                return Err(GuiError::host(
                    "drop policy protocol resolver requires a non-empty session id",
                ));
            }
            Ok(Self {
                session_id,
                exchange,
            })
        }

        pub fn session_id(&self) -> &str {
            &self.session_id
        }

        pub fn exchange(&self) -> &E {
            &self.exchange
        }

        pub fn exchange_mut(&mut self) -> &mut E {
            &mut self.exchange
        }

        pub fn into_exchange(self) -> E {
            self.exchange
        }
    }

    impl<E> SelfDrawnDropPolicyResolver for ProtocolDropPolicyResolverV1<E>
    where
        E: ProtocolDropPolicyExchangeV1,
    {
        fn resolve(&mut self, query: &SelfDrawnDropPolicyQuery) -> SelfDrawnDropPolicyResolution {
            let request = protocol_drop_policy_query_v1(&self.session_id, query);
            match self.exchange.exchange(request) {
                ProtocolDropPolicyExchangeResultV1::Response(response) => {
                    match self_drawn_drop_policy_response_from_v1(response, &self.session_id, query)
                    {
                        Ok(response) => SelfDrawnDropPolicyResolution::Resolved(response),
                        Err(_) => SelfDrawnDropPolicyResolution::Failed,
                    }
                }
                ProtocolDropPolicyExchangeResultV1::TimedOut => {
                    SelfDrawnDropPolicyResolution::TimedOut
                }
                ProtocolDropPolicyExchangeResultV1::Unavailable => {
                    SelfDrawnDropPolicyResolution::Unavailable
                }
                ProtocolDropPolicyExchangeResultV1::Failed => SelfDrawnDropPolicyResolution::Failed,
            }
        }
    }
}

#[cfg(feature = "platform-runtime")]
pub use self_drawn::{
    protocol_drop_policy_query_v1, self_drawn_drop_policy_response_from_v1,
    ProtocolDropPolicyExchangeResultV1, ProtocolDropPolicyExchangeV1, ProtocolDropPolicyResolverV1,
};

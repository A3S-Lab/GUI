//! Strict, transport-neutral protocol primitives for the optional TSX
//! authoring frontend.
//!
//! The application JavaScript runtime remains a separate Node process. This
//! module only owns bounded wire framing, handshake negotiation, and message
//! sequencing; it does not embed or depend on Node, Nub, N-API, Graphics, or an
//! operating-system widget toolkit.

mod framing;
mod handshake;
mod message;

pub use framing::*;
pub use handshake::*;
pub use message::*;

#[cfg(test)]
mod tests;

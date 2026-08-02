//! Strict, transport-neutral protocol primitives for the optional TSX
//! authoring frontend.
//!
//! The application JavaScript runtime remains a separate Node process. This
//! module owns strict wire DTOs, bounded framing, handshake negotiation, and
//! transactional application-message ordering; it does not embed or depend on
//! Node, Nub, N-API, or an operating-system widget toolkit. Optional
//! `platform-runtime` conversions consume the shared self-drawn records without
//! introducing a planned-widget response into the protocol core.

mod application;
mod framing;
mod handshake;
mod message;
mod session;

pub use application::*;
pub use framing::*;
pub use handshake::*;
pub use message::*;
pub use session::*;

#[cfg(test)]
mod tests;

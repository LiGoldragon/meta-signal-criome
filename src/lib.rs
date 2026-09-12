//! Meta Signal contract for Criome: privileged daemon control.
//!
//! Configuration, root founding, authorization approval, and intercept-policy
//! administration. The evaluated authorization surface itself belongs to the
//! ordinary contract, `signal-criome`, and is imported from it rather than
//! restated here.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, and the wire framing come from `signal`.

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv frame and its three kinds, re-exported from `signal` so
/// a frame of this contract is the same Rust type as every other contract's
/// frame and a consumer need not name `signal` itself to speak this contract.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");

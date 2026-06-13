//! Schema-derived meta signal contract for privileged `criome` daemon
//! configuration.
//!
//! Ordinary criome trust traffic (sign, verify, identity, attestation,
//! authorization) lives in `signal-criome`. This crate carries the meta plane:
//! the authenticated `Configure` operation that applies `criome`'s typed
//! daemon configuration.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl ConfigurationGeneration {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

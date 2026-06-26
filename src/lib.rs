//! Schema-derived meta signal contract for privileged `criome` daemon control.
//!
//! Ordinary criome trust traffic (sign, verify, identity, attestation,
//! authorization) lives in `signal-criome`. This crate carries the meta plane:
//! authenticated configuration, parked authorization observation, and
//! approval-by-slot operations.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl ConfigurationGeneration {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl InterceptPolicyObservation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InterceptPolicyObservation {
    fn default() -> Self {
        Self::new()
    }
}

impl InterceptPolicyStreamToken {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

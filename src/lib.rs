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

impl RootFoundingAcceptance {
    /// The owner's explicit founding accept: the self-certifying `anchor` plus
    /// the full `cohort` (RootGenesis) so the daemon re-derives and matches the
    /// anchor before its master key emits an attached founding signature.
    pub fn new(anchor: RootAnchorDigest, cohort: RootGenesis) -> Self {
        Self {
            root_anchor_digest: anchor,
            root_genesis: cohort,
        }
    }
}

impl RootFoundingAccepted {
    /// The founding acknowledgment: the accepted `anchor` and this node's
    /// attached, scheme-tagged `FoundingSignature` over the founding statement.
    pub fn new(anchor: RootAnchorDigest, founding_signature: FoundingSignature) -> Self {
        Self {
            root_anchor_digest: anchor,
            founding_signature,
        }
    }
}

impl RootFoundingObservation {
    /// The owner's read of this node's founding progress and its
    /// pending-founding queue; the request carries no selector.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RootFoundingObservation {
    fn default() -> Self {
        Self::new()
    }
}

impl RootFoundingStatus {
    /// This node's founding `state` and the `pending` foundings awaiting an
    /// explicit owner accept.
    pub fn new(state: RootFoundingState, pending: Vec<PendingFounding>) -> Self {
        Self {
            root_founding_state: state,
            pending_founding_vector: pending,
        }
    }
}

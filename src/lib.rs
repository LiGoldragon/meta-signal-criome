//! Meta signal contract — privileged `criome` daemon configuration.
//!
//! Ordinary criome trust traffic (sign, verify, identity, attestation,
//! authorization) lives in `signal-criome`. This crate carries the meta plane:
//! the authenticated `Configure` operation that applies `criome`'s typed daemon
//! configuration — its socket and its `criome.sema` store location, and the
//! root-of-trust material that "Criome verifies; Persona decides" rests on.
//!
//! The basic meta operation of every component is daemon configuration. The
//! `CriomeDaemonConfiguration` here mirrors the daemon's startup record; it is
//! the canonical contract home for that configuration. The `criome` daemon
//! currently defines an equivalent shape locally and should adopt this type so
//! startup and meta reconfiguration share one record (pre-production
//! reconciliation, no compatibility constraint).

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DaemonSocketPath(String);

impl DaemonSocketPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DaemonStorePath(String);

impl DaemonStorePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CriomeDaemonConfiguration {
    pub socket_path: DaemonSocketPath,
    pub store_path: DaemonStorePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct ConfigurationGeneration(u64);

impl ConfigurationGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl NotaDecode for ConfigurationGeneration {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

impl NotaEncode for ConfigurationGeneration {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    StoreUnavailable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaCriome {
        operation Configure(CriomeDaemonConfiguration),
    }
    reply MetaCriomeReply {
        Configured(Configured),
        ConfigurationRejected(ConfigurationRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
}

impl From<CriomeDaemonConfiguration> for Operation {
    fn from(payload: CriomeDaemonConfiguration) -> Self {
        Self::Configure(payload)
    }
}

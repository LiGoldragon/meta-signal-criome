//! Meta signal contract - privileged `criome` daemon configuration.
//!
//! Ordinary criome trust traffic (sign, verify, identity, attestation,
//! authorization) lives in `signal-criome`. This crate carries the meta plane:
//! the authenticated `Configure` operation that applies `criome`'s typed daemon
//! configuration - its socket and its `criome.sema` store location, and the
//! root-of-trust material that "Criome verifies; Persona decides" rests on.
//!
//! The basic meta operation of every component is daemon configuration. The
//! `Configure` payload is `signal_criome::CriomeDaemonConfiguration` - the same
//! record the daemon decodes at binary startup - so startup and meta
//! reconfiguration share one definition that lives in the ordinary contract.

#[cfg(feature = "nota-text")]
use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
pub use signal_criome::CriomeDaemonConfiguration;
use signal_frame::signal_channel;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
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

#[cfg(feature = "nota-text")]
impl NotaDecode for ConfigurationGeneration {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

#[cfg(feature = "nota-text")]
impl NotaEncode for ConfigurationGeneration {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    StoreUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
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

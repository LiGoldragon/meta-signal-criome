// Handwritten operational behavior for the authority-verified owner Criome Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_criome::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_external_newtype!(z2VQPE, u64);
wire_enum!(z2Vdzj { unit { 0 => z2VM7Y : "Gathering", 1 => z2VMy3 : "Founded", 2 => z2VTT2 : "Unfounded" } unary {  } });
wire_struct!(z2VYg3 {  });
wire_struct!(z2VYYB { field_0: z2Vdzj, field_1: Vec< z2VP73> });
wire_enum!(z2VSN6 { unit { 0 => z2Vdyw : "MalformedGenesis", 1 => z2VWqd : "ManagerAuthorityRequired", 2 => z2VZDb : "CohortMismatch", 3 => z2VNcf : "AlreadyFounded" } unary {  } });
wire_enum!(z2VKtS { unit {  } unary { 0 => z2VR4u(z2VWz5) : "ObserveRootFounding", 1 => z2VcU1(signal_criome::schema::lib::z2VSGX) : "AnswerParkedRequest", 2 => z2VXTq(z2VYg3) : "ObserveInterceptPolicies", 3 => z2VZJf(z2VWbk) : "SubmitAuthorizationApproval", 4 => z2VNKR(signal_criome::schema::lib::z2VXWs) : "CancelInterceptPolicy", 5 => z2VYmK(signal_criome::schema::lib::z2VS9T) : "Configure", 6 => z2VYir(signal_criome::schema::lib::z2VdtA) : "ObserveParkedAuthorizations", 7 => z2VU18(signal_criome::schema::lib::z2VUjo) : "ReplaceInterceptPolicy", 8 => z2VbVk(z2VMDj) : "InitiateRootFounding", 9 => z2VYdA(signal_criome::schema::lib::z2VUjo) : "CreateInterceptPolicy", 10 => z2VVMU(z2VYDr) : "RetractInterceptPolicyObservation", 11 => z2Var1(z2VSjN) : "AcceptRootFounding", 12 => z2VMWd(signal_criome::schema::lib::z2VNo7) : "FetchParkedRequests", 13 => z2VX8U(z2VYg3) : "ListInterceptPolicies" } });
wire_struct!(z2VR26 { field_0: z2VVVL, field_1: z2VVm5 });
wire_struct!(z2VMDj { field_0: signal_criome::schema::lib::z2VSrE });
wire_struct!(z2VSjN { field_0: signal_criome::schema::lib::z2VWaA, field_1: signal_criome::schema::lib::z2VSrE });
wire_external_newtype!(z2VYDr, std::string::String);
wire_struct!(z2VYmf { field_0: z2VQPE });
wire_enum!(z2VNfL { unit { 0 => z2VcPz : "Defer", 1 => z2VevL : "Reject", 2 => z2VYjP : "Approve" } unary {  } });
wire_struct!(z2VWbk { field_0: signal_criome::schema::lib::z2VUph, field_1: z2VNfL });
wire_enum!(z2VVm5 { unit { 0 => z2VYmh : "DependencyNotReady", 1 => z2VYBX : "NotBuiltYet" } unary {  } });
wire_struct!(z2VXr4 { field_0: z2VZJN });
wire_struct!(z2VP73 { field_0: signal_criome::schema::lib::z2VWaA, field_1: signal_criome::schema::lib::z2VSrE, field_2: signal_criome::schema::lib::z2VL5X });
wire_struct!(z2VXNG { field_0: signal_criome::schema::lib::z2VWaA, field_1: signal_criome::schema::lib::z2VPfK });
wire_struct!(z2VUHx { field_0: signal_criome::schema::lib::z2VUph, field_1: z2VNfL });
wire_struct!(z2VWz5 {  });
wire_enum!(z2VZJN { unit { 0 => z2VdFh : "ManagerAuthorityRequired", 1 => z2VMLC : "StoreUnavailable", 2 => z2VT22 : "MalformedConfiguration" } unary {  } });
wire_enum!(z2VUG9 { unit {  } unary { 0 => z2VbK6(z2VTPp) : "RootFoundingRefused", 1 => z2VMVt(z2VYmf) : "ConfigurationApplied", 2 => z2VS5r(z2VUHx) : "AuthorizationApprovalStored", 3 => z2VWCn(z2VR26) : "OperationUnimplemented", 4 => z2VPTr(signal_criome::schema::lib::z2VShF) : "InterceptPolicyCancelled", 5 => z2VdGy(z2VYDr) : "InterceptPolicyObservationRetracted", 6 => z2VS59(signal_criome::schema::lib::z2Vb3U) : "InterceptPoliciesListed", 7 => z2VXBy(signal_criome::schema::lib::z2VfB8) : "InterceptPolicyReplaced", 8 => z2VTAm(signal_criome::schema::lib::z2Vb3U) : "InterceptPolicyObservationOpened", 9 => z2Veff(signal_criome::schema::lib::z2VRAT) : "ParkedRequestsFetched", 10 => z2VQJa(signal_criome::schema::lib::z2VW3f) : "ParkedRequestAnswered", 11 => z2VYP9(signal_criome::schema::lib::z2VfB8) : "InterceptPolicyCreated", 12 => z2VP45(signal_criome::schema::lib::z2VUEA) : "ParkedAuthorizations", 13 => z2VKp1(z2VXr4) : "ConfigurationRefused", 14 => z2VYiQ(z2VXNG) : "RootFoundingConfirmed", 15 => z2VMm2(z2VYYB) : "RootFoundingObserved" } });
wire_enum!(z2VRAr { unit {  } unary { 0 => z2VR4c(signal_criome::schema::lib::z2VfB8) : "Created", 1 => z2Va6Z(signal_criome::schema::lib::z2VfB8) : "Replaced", 2 => z2VLtX(signal_criome::schema::lib::z2VShF) : "Cancelled" } });
wire_enum!(z2VVVL { unit { 0 => z2VMy4 : "Configure", 1 => z2VSiM : "CreateInterceptPolicy", 2 => z2VeK1 : "InitiateRootFounding", 3 => z2VM1b : "ReplaceInterceptPolicy", 4 => z2VR3N : "FetchParkedRequests", 5 => z2Vbaa : "CancelInterceptPolicy", 6 => z2VLYL : "ObserveInterceptPolicies", 7 => z2VMHx : "AnswerParkedRequest", 8 => z2VerJ : "ObserveRootFounding", 9 => z2VLVL : "AcceptRootFounding", 10 => z2VcS1 : "SubmitAuthorizationApproval", 11 => z2Vb59 : "RetractInterceptPolicyObservation", 12 => z2VLLW : "ObserveParkedAuthorizations", 13 => z2VTPW : "ListInterceptPolicies" } unary {  } });
wire_struct!(z2VTPp { field_0: z2VSN6 });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_criome::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_criome::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_criome::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VQPE);
archive_root!(z2Vdzj);
archive_root!(z2VYg3);
archive_root!(z2VYYB);
archive_root!(z2VSN6);
archive_root!(z2VKtS);
archive_root!(z2VR26);
archive_root!(z2VMDj);
archive_root!(z2VSjN);
archive_root!(z2VYDr);
archive_root!(z2VYmf);
archive_root!(z2VNfL);
archive_root!(z2VWbk);
archive_root!(z2VVm5);
archive_root!(z2VXr4);
archive_root!(z2VP73);
archive_root!(z2VXNG);
archive_root!(z2VUHx);
archive_root!(z2VWz5);
archive_root!(z2VZJN);
archive_root!(z2VUG9);
archive_root!(z2VRAr);
archive_root!(z2VVVL);
archive_root!(z2VTPp);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(4) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    ObserveRootFounding,
    AnswerParkedRequest,
    ObserveInterceptPolicies,
    SubmitAuthorizationApproval,
    CancelInterceptPolicy,
    Configure,
    ObserveParkedAuthorizations,
    ReplaceInterceptPolicy,
    InitiateRootFounding,
    CreateInterceptPolicy,
    RetractInterceptPolicyObservation,
    AcceptRootFounding,
    FetchParkedRequests,
    ListInterceptPolicies,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    RootFoundingRefused,
    ConfigurationApplied,
    AuthorizationApprovalStored,
    OperationUnimplemented,
    InterceptPolicyCancelled,
    InterceptPolicyObservationRetracted,
    InterceptPoliciesListed,
    InterceptPolicyReplaced,
    InterceptPolicyObservationOpened,
    ParkedRequestsFetched,
    ParkedRequestAnswered,
    InterceptPolicyCreated,
    ParkedAuthorizations,
    ConfigurationRefused,
    RootFoundingConfirmed,
    RootFoundingObserved,
}

impl z2VKtS {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VR4u(_) => InputRoute::ObserveRootFounding,
            Self::z2VcU1(_) => InputRoute::AnswerParkedRequest,
            Self::z2VXTq(_) => InputRoute::ObserveInterceptPolicies,
            Self::z2VZJf(_) => InputRoute::SubmitAuthorizationApproval,
            Self::z2VNKR(_) => InputRoute::CancelInterceptPolicy,
            Self::z2VYmK(_) => InputRoute::Configure,
            Self::z2VYir(_) => InputRoute::ObserveParkedAuthorizations,
            Self::z2VU18(_) => InputRoute::ReplaceInterceptPolicy,
            Self::z2VbVk(_) => InputRoute::InitiateRootFounding,
            Self::z2VYdA(_) => InputRoute::CreateInterceptPolicy,
            Self::z2VVMU(_) => InputRoute::RetractInterceptPolicyObservation,
            Self::z2Var1(_) => InputRoute::AcceptRootFounding,
            Self::z2VMWd(_) => InputRoute::FetchParkedRequests,
            Self::z2VX8U(_) => InputRoute::ListInterceptPolicies,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VUG9 {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VbK6(_) => OutputRoute::RootFoundingRefused,
            Self::z2VMVt(_) => OutputRoute::ConfigurationApplied,
            Self::z2VS5r(_) => OutputRoute::AuthorizationApprovalStored,
            Self::z2VWCn(_) => OutputRoute::OperationUnimplemented,
            Self::z2VPTr(_) => OutputRoute::InterceptPolicyCancelled,
            Self::z2VdGy(_) => OutputRoute::InterceptPolicyObservationRetracted,
            Self::z2VS59(_) => OutputRoute::InterceptPoliciesListed,
            Self::z2VXBy(_) => OutputRoute::InterceptPolicyReplaced,
            Self::z2VTAm(_) => OutputRoute::InterceptPolicyObservationOpened,
            Self::z2Veff(_) => OutputRoute::ParkedRequestsFetched,
            Self::z2VQJa(_) => OutputRoute::ParkedRequestAnswered,
            Self::z2VYP9(_) => OutputRoute::InterceptPolicyCreated,
            Self::z2VP45(_) => OutputRoute::ParkedAuthorizations,
            Self::z2VKp1(_) => OutputRoute::ConfigurationRefused,
            Self::z2VYiQ(_) => OutputRoute::RootFoundingConfirmed,
            Self::z2VMm2(_) => OutputRoute::RootFoundingObserved,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VKtS {}

impl signal_frame::SignalOperationHeads for z2VKtS {
    const HEADS: &'static [&'static str] = &["ObserveRootFounding", "AnswerParkedRequest", "ObserveInterceptPolicies", "SubmitAuthorizationApproval", "CancelInterceptPolicy", "Configure", "ObserveParkedAuthorizations", "ReplaceInterceptPolicy", "InitiateRootFounding", "CreateInterceptPolicy", "RetractInterceptPolicyObservation", "AcceptRootFounding", "FetchParkedRequests", "ListInterceptPolicies"];
}

impl signal_frame::LogVariant for z2VKtS {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VKtS, z2VUG9>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VKtS, z2VUG9>;
pub type Request = signal_frame::Request<z2VKtS>;
pub type ReplyEnvelope = signal_frame::Reply<z2VUG9>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VKtS>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VKtS), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}

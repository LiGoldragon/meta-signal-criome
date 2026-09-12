#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ConfigurationGeneration = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RootFoundingState {
    Gathering,
    Founded,
    Unfounded,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterceptPolicyObservation {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingStatus {
    pub root_founding_state: RootFoundingState,
    pub pending_founding_vector: std::vec::Vec<PendingFounding>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RootFoundingRejectionReason {
    MalformedGenesis,
    ManagerAuthorityRequired,
    CohortMismatch,
    AlreadyFounded,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RequestUnimplemented {
    pub operation_kind: OperationKind,
    pub unimplemented_reason: UnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingInitiation {
    pub root_genesis: signal_criome::RootGenesis,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingAcceptance {
    pub root_anchor_digest: signal_criome::RootAnchorDigest,
    pub root_genesis: signal_criome::RootGenesis,
}
#[rustfmt::skip]
pub type InterceptPolicyStreamToken = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Configured {
    pub configuration_generation: ConfigurationGeneration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AuthorizationApprovalDecision {
    Defer,
    Reject,
    Approve,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AuthorizationApproval {
    pub authorization_request_slot: signal_criome::AuthorizationRequestSlot,
    pub authorization_approval_decision: AuthorizationApprovalDecision,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnimplementedReason {
    DependencyNotReady,
    NotBuiltYet,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationRejected {
    pub configuration_rejection_reason: ConfigurationRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PendingFounding {
    pub root_anchor_digest: signal_criome::RootAnchorDigest,
    pub root_genesis: signal_criome::RootGenesis,
    pub identity: signal_criome::Identity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingAccepted {
    pub root_anchor_digest: signal_criome::RootAnchorDigest,
    pub founding_signature: signal_criome::FoundingSignature,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AuthorizationApprovalRecorded {
    pub authorization_request_slot: signal_criome::AuthorizationRequestSlot,
    pub authorization_approval_decision: AuthorizationApprovalDecision,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingObservation {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    StoreUnavailable,
    MalformedConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InterceptPolicyChange {
    Created(signal_criome::InterceptPolicy),
    Replaced(signal_criome::InterceptPolicy),
    Cancelled(signal_criome::InterceptPolicyIdentifier),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OperationKind {
    Configure,
    CreateInterceptPolicy,
    InitiateRootFounding,
    ReplaceInterceptPolicy,
    FetchParkedRequests,
    CancelInterceptPolicy,
    ObserveInterceptPolicies,
    AnswerParkedRequest,
    ObserveRootFounding,
    AcceptRootFounding,
    SubmitAuthorizationApproval,
    RetractInterceptPolicyObservation,
    ObserveParkedAuthorizations,
    ListInterceptPolicies,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RootFoundingRejected {
    pub root_founding_rejection_reason: RootFoundingRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    ObserveRootFounding(RootFoundingObservation),
    AnswerParkedRequest(signal_criome::ParkedRequestAnswer),
    ObserveInterceptPolicies(InterceptPolicyObservation),
    SubmitAuthorizationApproval(AuthorizationApproval),
    CancelInterceptPolicy(signal_criome::InterceptPolicyCancellation),
    Configure(signal_criome::CriomeDaemonConfiguration),
    ObserveParkedAuthorizations(signal_criome::ParkedAuthorizationObservation),
    ReplaceInterceptPolicy(signal_criome::InterceptPolicyProposal),
    InitiateRootFounding(RootFoundingInitiation),
    CreateInterceptPolicy(signal_criome::InterceptPolicyProposal),
    RetractInterceptPolicyObservation(InterceptPolicyStreamToken),
    AcceptRootFounding(RootFoundingAcceptance),
    FetchParkedRequests(signal_criome::ParkedRequestQuery),
    ListInterceptPolicies(InterceptPolicyObservation),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    RootFoundingRefused(RootFoundingRejected),
    ConfigurationApplied(Configured),
    AuthorizationApprovalStored(AuthorizationApprovalRecorded),
    OperationUnimplemented(RequestUnimplemented),
    InterceptPolicyCancelled(signal_criome::InterceptPolicyIdentifier),
    InterceptPolicyObservationRetracted(InterceptPolicyStreamToken),
    InterceptPoliciesListed(signal_criome::ActiveInterceptPolicies),
    InterceptPolicyReplaced(signal_criome::InterceptPolicy),
    InterceptPolicyObservationOpened(signal_criome::ActiveInterceptPolicies),
    ParkedRequestsFetched(signal_criome::ParkedRequestSnapshot),
    ParkedRequestAnswered(signal_criome::ParkedRequestResolution),
    InterceptPolicyCreated(signal_criome::InterceptPolicy),
    ParkedAuthorizations(signal_criome::ParkedAuthorizationSnapshot),
    ConfigurationRefused(ConfigurationRejected),
    RootFoundingConfirmed(RootFoundingAccepted),
    RootFoundingObserved(RootFoundingStatus),
}

//! Round-trip witnesses for the schema-derived criome meta contract.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, AuthorizationApprovalRecorded,
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    CriomeDaemonConfiguration, Frame, FrameBody, Input, InterceptPolicyObservation,
    InterceptPolicyStreamToken, OperationKind, Output, PendingFounding, RequestUnimplemented,
    RootFoundingInitiation, RootFoundingObservation, RootFoundingState, RootFoundingStatus,
    UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    ActiveInterceptPolicies, ApprovalAuditSource, AttestedMoment, AttestedMomentProposition,
    AuthorizationEvaluation, AuthorizationRequestSlot, AuthorizedObjectKind,
    AuthorizedObjectReference, BlsPublicKey, ComponentKind, Contract, ContractDigest, Evidence,
    ExpiryAction, FoundingMember, GenesisDomainTag, Identity, InterceptPolicy,
    InterceptPolicyCancellation, InterceptPolicyIdentifier, InterceptPolicyProposal,
    InterceptPolicyWindow, InterceptTargetSelector, MentciSessionSlot, ObjectDigest,
    OperationDigest, ParkedAuthorization, ParkedAuthorizationObservation,
    ParkedAuthorizationSnapshot, ParkedRequestAnswer, ParkedRequestDecision,
    ParkedRequestIdentifier, ParkedRequestOutcome, ParkedRequestQuery, ParkedRequestResolution,
    ParkedRequestSnapshot, ParkedSpiritRequest, PolicyDurationNanos, PolicyMember,
    PolicyOverlapMode, PolicyPriority, PrincipalName, RawSpiritOperationPayload, ReplayNonce,
    RequiredSignatureThreshold, RootAnchorDigest, RootGenesis, Rule, SpiritAuthorizationContext,
    SpiritOperationName, SpiritOperationNames, SpiritProcessKey, Threshold, TimeWindow,
    TimestampNanos,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn configuration() -> CriomeDaemonConfiguration {
    CriomeDaemonConfiguration::new("/run/criome/criome.sock", "/var/lib/criome/criome.sema")
}

fn founding_member(name: &str) -> FoundingMember {
    FoundingMember::new(
        Identity::Host(PrincipalName::new(name)),
        BlsPublicKey::new(format!("{name}-master-pubkey")),
    )
}

fn root_genesis() -> RootGenesis {
    RootGenesis::new(
        Contract::root(Rule::threshold_rule(Threshold::new(
            RequiredSignatureThreshold::new(2),
            vec![
                PolicyMember::key_member(Identity::Host(PrincipalName::new("mirror-alpha"))),
                PolicyMember::key_member(Identity::Host(PrincipalName::new("mirror-beta"))),
            ],
        ))),
        vec![
            founding_member("mirror-alpha"),
            founding_member("mirror-beta"),
        ],
        GenesisDomainTag::CriomeRootFoundingV1,
        ReplayNonce::new("genesis-nonce-1"),
    )
}

fn root_anchor() -> RootAnchorDigest {
    RootAnchorDigest::new(ObjectDigest::new("root-anchor-1"))
}

fn root_founding_status() -> RootFoundingStatus {
    RootFoundingStatus {
        root_founding_state: RootFoundingState::Gathering,
        pending_founding_vector: vec![PendingFounding {
            root_anchor_digest: root_anchor(),
            root_genesis: root_genesis(),
            identity: Identity::Host(PrincipalName::new("mirror-alpha")),
        }],
    }
}

fn evaluation() -> AuthorizationEvaluation {
    let operation = OperationDigest::from_bytes(b"mentci-meta-approval");
    AuthorizationEvaluation {
        contract_digest: ContractDigest::from_bytes(b"approval-contract"),
        authorized_object_reference: AuthorizedObjectReference {
            component_kind: ComponentKind::Spirit,
            object_digest: operation.object_digest().clone(),
            authorized_object_kind: AuthorizedObjectKind::Head,
        },
        evidence: Evidence::new(
            ComponentKind::Spirit,
            operation,
            AttestedMoment::new(
                AttestedMomentProposition::new(
                    TimeWindow {
                        opens_at: TimestampNanos::new(1),
                        closes_at: TimestampNanos::new(2),
                    },
                    RequiredSignatureThreshold::new(1),
                    Vec::new(),
                ),
                Vec::new(),
            ),
            Vec::new(),
            Vec::new(),
        ),
    }
}

fn request_slot() -> AuthorizationRequestSlot {
    AuthorizationRequestSlot::new("authorization-request-1")
}

fn mentci_session_slot() -> MentciSessionSlot {
    MentciSessionSlot::new("mentci-session-1")
}

fn intercept_policy_identifier() -> InterceptPolicyIdentifier {
    InterceptPolicyIdentifier::new("intercept-policy-1")
}

fn spirit_process_key() -> SpiritProcessKey {
    SpiritProcessKey::new("spirit-process-main")
}

fn intercept_target() -> InterceptTargetSelector {
    InterceptTargetSelector::new(spirit_process_key())
}

fn spirit_operation_names() -> SpiritOperationNames {
    SpiritOperationNames::from_names(vec![
        SpiritOperationName::new("Record"),
        SpiritOperationName::new("ChangeRecord"),
    ])
}

fn intercept_policy_proposal() -> InterceptPolicyProposal {
    InterceptPolicyProposal {
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        policy_duration_nanos: PolicyDurationNanos::new(100),
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(50),
        policy_overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
    }
}

fn intercept_policy() -> InterceptPolicy {
    InterceptPolicy {
        intercept_policy_identifier: intercept_policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        intercept_policy_window: InterceptPolicyWindow {
            starts_at: TimestampNanos::new(100),
            expires_at: TimestampNanos::new(200),
        },
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(50),
    }
}

fn active_intercept_policies() -> ActiveInterceptPolicies {
    ActiveInterceptPolicies::from_policies(vec![intercept_policy()])
}

fn parked_request_query() -> ParkedRequestQuery {
    ParkedRequestQuery {
        optional_mentci_session_slot: Some(mentci_session_slot()),
        optional_intercept_target_selector: Some(intercept_target()),
    }
}

fn parked_request_answer() -> ParkedRequestAnswer {
    ParkedRequestAnswer {
        parked_request_identifier: ParkedRequestIdentifier::new("parked-request-1"),
        parked_request_decision: ParkedRequestDecision::Approve,
    }
}

fn parked_spirit_request() -> ParkedSpiritRequest {
    ParkedSpiritRequest {
        parked_request_identifier: ParkedRequestIdentifier::new("parked-request-1"),
        intercept_policy_identifier: intercept_policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        spirit_authorization_context: SpiritAuthorizationContext {
            spirit_operation_name: SpiritOperationName::new("Record"),
            raw_spirit_operation_payload: RawSpiritOperationPayload::new("(Record (...))"),
            spirit_process_key: spirit_process_key(),
        },
        parked_at: TimestampNanos::new(120),
        expires_at: TimestampNanos::new(200),
        expiry_action: ExpiryAction::AutoApprove,
    }
}

fn parked_request_snapshot() -> ParkedRequestSnapshot {
    ParkedRequestSnapshot::from_requests(vec![parked_spirit_request()])
}

fn parked_request_resolution() -> ParkedRequestResolution {
    ParkedRequestResolution {
        parked_request_identifier: ParkedRequestIdentifier::new("parked-request-1"),
        intercept_policy_identifier: intercept_policy_identifier(),
        parked_request_outcome: ParkedRequestOutcome::Approved,
        approval_audit_source: ApprovalAuditSource::Manual,
        timestamp_nanos: TimestampNanos::new(130),
    }
}

fn assert_request_round_trips(request: Input) {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => assert_eq!(decoded_request.payloads().head(), &request),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn assert_reply_round_trips(reply: Output) {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
    match decoded.into_body() {
        FrameBody::Reply {
            reply: decoded_reply,
            ..
        } => match decoded_reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => assert_eq!(payload, reply),
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[cfg(feature = "nota-text")]
fn assert_nota_round_trips<Value>(value: &Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(&recovered, value);
}

#[test]
fn configure_request_carries_the_signal_criome_configuration_type() {
    let request = Input::Configure(configuration());
    assert_request_round_trips(request.clone());
    #[cfg(feature = "nota-text")]
    assert_nota_round_trips(&request);
}

#[test]
fn authorization_approval_request_round_trips() {
    let request = Input::SubmitAuthorizationApproval(AuthorizationApproval {
        authorization_request_slot: request_slot(),
        authorization_approval_decision: AuthorizationApprovalDecision::Approve,
    });
    assert_request_round_trips(request.clone());
    #[cfg(feature = "nota-text")]
    assert_nota_round_trips(&request);
}

#[test]
fn intercept_policy_owner_requests_round_trip() {
    let requests = [
        Input::CreateInterceptPolicy(intercept_policy_proposal()),
        Input::ReplaceInterceptPolicy(intercept_policy_proposal()),
        Input::CancelInterceptPolicy(InterceptPolicyCancellation::new(
            intercept_policy_identifier(),
        )),
        Input::ListInterceptPolicies(InterceptPolicyObservation::new()),
        Input::ObserveInterceptPolicies(InterceptPolicyObservation::new()),
        Input::RetractInterceptPolicyObservation(InterceptPolicyStreamToken::new(
            "intercept-policy-stream-1",
        )),
        Input::FetchParkedRequests(parked_request_query()),
        Input::AnswerParkedRequest(parked_request_answer()),
    ];

    for request in requests {
        assert_request_round_trips(request.clone());
        #[cfg(feature = "nota-text")]
        assert_nota_round_trips(&request);
    }
}

#[test]
fn reply_variants_round_trip() {
    let replies = [
        Output::configuration_applied(ConfigurationGeneration::new(7)),
        Output::InterceptPolicyCreated(intercept_policy()),
        Output::InterceptPolicyReplaced(intercept_policy()),
        Output::InterceptPolicyCancelled(intercept_policy_identifier()),
        Output::InterceptPoliciesListed(active_intercept_policies()),
        Output::InterceptPolicyObservationOpened(active_intercept_policies()),
        Output::InterceptPolicyObservationRetracted(InterceptPolicyStreamToken::new(
            "intercept-policy-stream-1",
        )),
        Output::ParkedRequestsFetched(parked_request_snapshot()),
        Output::ParkedRequestAnswered(parked_request_resolution()),
        Output::parked_authorizations(ParkedAuthorizationSnapshot::from_parked(vec![
            ParkedAuthorization::from_evaluation(request_slot(), evaluation()),
        ])),
        Output::authorization_approval_stored(AuthorizationApprovalRecorded {
            authorization_request_slot: request_slot(),
            authorization_approval_decision: AuthorizationApprovalDecision::Approve,
        }),
        Output::ConfigurationRefused(ConfigurationRejected::new(
            ConfigurationRejectionReason::ManagerAuthorityRequired,
        )),
        Output::OperationUnimplemented(RequestUnimplemented {
            operation_kind: OperationKind::Configure,
            unimplemented_reason: UnimplementedReason::DependencyNotReady,
        }),
    ];
    for reply in replies {
        assert_reply_round_trips(reply.clone());
        #[cfg(feature = "nota-text")]
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn cross_node_founding_meta_ops_round_trip() {
    let requests = [
        Input::InitiateRootFounding(RootFoundingInitiation::new(root_genesis())),
        Input::ObserveRootFounding(RootFoundingObservation::new()),
    ];
    for request in requests {
        assert_request_round_trips(request.clone());
        #[cfg(feature = "nota-text")]
        assert_nota_round_trips(&request);
    }

    let reply = Output::RootFoundingObserved(root_founding_status());
    assert_reply_round_trips(reply.clone());
    #[cfg(feature = "nota-text")]
    assert_nota_round_trips(&reply);
}

#[test]
fn parked_authorization_observation_request_round_trips() {
    let request = Input::ObserveParkedAuthorizations(ParkedAuthorizationObservation::new());
    assert_request_round_trips(request.clone());
    #[cfg(feature = "nota-text")]
    assert_nota_round_trips(&request);
}

#[test]
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

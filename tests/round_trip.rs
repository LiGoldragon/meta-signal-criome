//! Round-trip witnesses for the schema-derived criome meta contract.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, AuthorizationApprovalRecorded,
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    CriomeDaemonConfiguration, Frame, FrameBody, Input, InterceptPolicyObservation,
    InterceptPolicyStreamToken, OperationKind, Output, RequestUnimplemented, UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    ActiveInterceptPolicies, ApprovalAuditSource, AttestedMoment, AttestedMomentProposition,
    AuthorizationEvaluation, AuthorizationRequestSlot, AuthorizedObjectKind,
    AuthorizedObjectReference, ComponentKind, ContractDigest, Evidence, ExpiryAction,
    InterceptPolicy, InterceptPolicyCancellation, InterceptPolicyIdentifier,
    InterceptPolicyProposal, InterceptPolicyWindow, InterceptTargetSelector, MentciSessionSlot,
    OperationDigest, ParkedAuthorization, ParkedAuthorizationObservation,
    ParkedAuthorizationSnapshot, ParkedRequestAnswer, ParkedRequestDecision,
    ParkedRequestIdentifier, ParkedRequestOutcome, ParkedRequestQuery, ParkedRequestResolution,
    ParkedRequestSnapshot, ParkedSpiritRequest, PolicyDurationNanos, PolicyOverlapMode,
    PolicyPriority, RawSpiritOperationPayload, RequiredSignatureThreshold,
    SpiritAuthorizationContext, SpiritOperationName, SpiritOperationNames, SpiritProcessKey,
    TimeWindow, TimestampNanos,
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

fn evaluation() -> AuthorizationEvaluation {
    let operation = OperationDigest::from_bytes(b"mentci-meta-approval");
    AuthorizationEvaluation {
        contract: ContractDigest::from_bytes(b"approval-contract"),
        object: AuthorizedObjectReference {
            component: ComponentKind::Spirit,
            digest: operation.object_digest().clone(),
            kind: AuthorizedObjectKind::Head,
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
        session_slot: mentci_session_slot(),
        target: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        duration: PolicyDurationNanos::new(100),
        expiry_action: ExpiryAction::AutoApprove,
        priority: PolicyPriority::new(50),
        overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
    }
}

fn intercept_policy() -> InterceptPolicy {
    InterceptPolicy {
        identifier: intercept_policy_identifier(),
        session_slot: mentci_session_slot(),
        target: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        window: InterceptPolicyWindow {
            starts_at: TimestampNanos::new(100),
            expires_at: TimestampNanos::new(200),
        },
        expiry_action: ExpiryAction::AutoApprove,
        priority: PolicyPriority::new(50),
    }
}

fn active_intercept_policies() -> ActiveInterceptPolicies {
    ActiveInterceptPolicies::from_policies(vec![intercept_policy()])
}

fn parked_request_query() -> ParkedRequestQuery {
    ParkedRequestQuery {
        session_slot: Some(mentci_session_slot()),
        target: Some(intercept_target()),
    }
}

fn parked_request_answer() -> ParkedRequestAnswer {
    ParkedRequestAnswer {
        identifier: ParkedRequestIdentifier::new("parked-request-1"),
        decision: ParkedRequestDecision::Approve,
    }
}

fn parked_spirit_request() -> ParkedSpiritRequest {
    ParkedSpiritRequest {
        identifier: ParkedRequestIdentifier::new("parked-request-1"),
        matched_policy: intercept_policy_identifier(),
        session_slot: mentci_session_slot(),
        context: SpiritAuthorizationContext {
            operation_name: SpiritOperationName::new("Record"),
            raw_payload: RawSpiritOperationPayload::new("(Record (...))"),
            target_key: spirit_process_key(),
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
        identifier: ParkedRequestIdentifier::new("parked-request-1"),
        matched_policy: intercept_policy_identifier(),
        outcome: ParkedRequestOutcome::Approved,
        audit_source: ApprovalAuditSource::Manual,
        resolved_at: TimestampNanos::new(130),
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
        request_slot: request_slot(),
        decision: AuthorizationApprovalDecision::Approve,
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
        Output::configured(ConfigurationGeneration::new(7)),
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
        Output::parked_authorization_snapshot(ParkedAuthorizationSnapshot::from_parked(vec![
            ParkedAuthorization::from_evaluation(request_slot(), evaluation()),
        ])),
        Output::authorization_approval_recorded(AuthorizationApprovalRecorded {
            request_slot: request_slot(),
            decision: AuthorizationApprovalDecision::Approve,
        }),
        Output::ConfigurationRejected(ConfigurationRejected::new(
            ConfigurationRejectionReason::ManagerAuthorityRequired,
        )),
        Output::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Configure,
            reason: UnimplementedReason::DependencyNotReady,
        }),
    ];
    for reply in replies {
        assert_reply_round_trips(reply.clone());
        #[cfg(feature = "nota-text")]
        assert_nota_round_trips(&reply);
    }
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

//! Round-trip witnesses for the schema-derived criome meta contract.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, ConfigurationGeneration,
    ConfigurationRejected, ConfigurationRejectionReason, CriomeDaemonConfiguration, Frame,
    FrameBody, Input, OperationKind, Output, RequestUnimplemented, UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    AttestedMoment, AttestedMomentProposition, AuthorizationEvaluated, AuthorizationEvaluation,
    AuthorizedObjectKind, AuthorizedObjectReference, ComponentKind, ContractDigest,
    EvaluationDecision, Evidence, OperationDigest, RequiredSignatureThreshold, TimeWindow,
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
        evaluation: evaluation(),
        decision: AuthorizationApprovalDecision::Approve,
    });
    assert_request_round_trips(request.clone());
    #[cfg(feature = "nota-text")]
    assert_nota_round_trips(&request);
}

#[test]
fn reply_variants_round_trip() {
    let replies = [
        Output::configured(ConfigurationGeneration::new(7)),
        Output::authorization_approval_recorded(AuthorizationEvaluated {
            contract: ContractDigest::from_bytes(b"approval-contract"),
            decision: EvaluationDecision::Authorized,
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
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

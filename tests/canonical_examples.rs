//! Canonical NOTA examples round-trip witness.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, AuthorizationApprovalRecorded,
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    CriomeDaemonConfiguration, Input, OperationKind, Output, RequestUnimplemented,
    UnimplementedReason,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    AttestedMoment, AttestedMomentProposition, AuthorizationEvaluation, AuthorizationRequestSlot,
    AuthorizedObjectKind, AuthorizedObjectReference, ComponentKind, ContractDigest, Evidence,
    ObjectDigest, OperationDigest, ParkedAuthorization, ParkedAuthorizationObservation,
    ParkedAuthorizationSnapshot, RequiredSignatureThreshold, TimeWindow, TimestampNanos,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn configuration() -> CriomeDaemonConfiguration {
    CriomeDaemonConfiguration::new("/run/criome/criome.sock", "/var/lib/criome/criome.sema")
}

fn request_slot() -> AuthorizationRequestSlot {
    AuthorizationRequestSlot::new("authorization-request-1")
}

fn evaluation() -> AuthorizationEvaluation {
    let operation = OperationDigest::new(ObjectDigest::new("operation-digest-1"));
    AuthorizationEvaluation {
        contract: ContractDigest::new(ObjectDigest::new("contract-digest-1")),
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
                        opens_at: TimestampNanos::new(10),
                        closes_at: TimestampNanos::new(20),
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

fn round_trip<Value>(value: Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let decoded = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(decoded, value, "decode for {text}");
    assert!(
        CANONICAL.contains(&text),
        "examples/canonical.nota missing line: {text}",
    );
}

#[test]
fn canonical_input_examples_round_trip() {
    round_trip(Input::Configure(configuration()));
    round_trip(Input::ObserveParkedAuthorizations(
        ParkedAuthorizationObservation::new(),
    ));
    round_trip(Input::SubmitAuthorizationApproval(AuthorizationApproval {
        request_slot: request_slot(),
        decision: AuthorizationApprovalDecision::Approve,
    }));
}

#[test]
fn canonical_output_examples_round_trip() {
    round_trip(Output::Configured(ConfigurationGeneration::new(7).into()));
    round_trip(Output::ParkedAuthorizationSnapshot(
        ParkedAuthorizationSnapshot::from_parked(vec![ParkedAuthorization::from_evaluation(
            request_slot(),
            evaluation(),
        )]),
    ));
    round_trip(Output::AuthorizationApprovalRecorded(
        AuthorizationApprovalRecorded {
            request_slot: request_slot(),
            decision: AuthorizationApprovalDecision::Approve,
        },
    ));
    round_trip(Output::ConfigurationRejected(ConfigurationRejected::new(
        ConfigurationRejectionReason::ManagerAuthorityRequired,
    )));
    round_trip(Output::RequestUnimplemented(RequestUnimplemented {
        operation: OperationKind::Configure,
        reason: UnimplementedReason::DependencyNotReady,
    }));
}

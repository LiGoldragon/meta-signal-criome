//! Canonical NOTA examples round-trip witness.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, AuthorizationApprovalRecorded,
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    CriomeDaemonConfiguration, Input, OperationKind, Output, PendingFounding, RequestUnimplemented,
    RootFoundingAcceptance, RootFoundingAccepted, RootFoundingInitiation, RootFoundingObservation,
    RootFoundingRejected, RootFoundingRejectionReason, RootFoundingState, RootFoundingStatus,
    UnimplementedReason,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    AttestedMoment, AttestedMomentProposition, AuthorizationEvaluation, AuthorizationRequestSlot,
    AuthorizedObjectKind, AuthorizedObjectReference, BlsPublicKey, BlsSignature, ComponentKind,
    Contract, ContractDigest, Evidence, FoundingMember, FoundingSignature, GenesisDomainTag,
    Identity, ObjectDigest, OperationDigest, ParkedAuthorization, ParkedAuthorizationObservation,
    ParkedAuthorizationSnapshot, PolicyMember, PrincipalName, ReplayNonce,
    RequiredSignatureThreshold, RootAnchorDigest, RootGenesis, Rule, SignatureEnvelope,
    SignatureScheme, Threshold, TimeWindow, TimestampNanos,
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
        contract_digest: ContractDigest::new(ObjectDigest::new("contract-digest-1")),
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

fn founding_signature() -> FoundingSignature {
    FoundingSignature::new(
        Identity::Host(PrincipalName::new("mirror-alpha")),
        SignatureEnvelope {
            signature_scheme: SignatureScheme::Bls12_381MinPk,
            bls_public_key: BlsPublicKey::new("public-key-1"),
            bls_signature: BlsSignature::new("signature-1"),
        },
    )
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
        authorization_request_slot: request_slot(),
        authorization_approval_decision: AuthorizationApprovalDecision::Approve,
    }));
    round_trip(Input::AcceptRootFounding(RootFoundingAcceptance::new(
        root_anchor(),
        root_genesis(),
    )));
    round_trip(Input::InitiateRootFounding(RootFoundingInitiation::new(
        root_genesis(),
    )));
    round_trip(Input::ObserveRootFounding(RootFoundingObservation::new()));
}

#[test]
fn canonical_output_examples_round_trip() {
    round_trip(Output::ConfigurationApplied(
        ConfigurationGeneration::new(7).into(),
    ));
    round_trip(Output::ParkedAuthorizations(
        ParkedAuthorizationSnapshot::from_parked(vec![ParkedAuthorization::from_evaluation(
            request_slot(),
            evaluation(),
        )]),
    ));
    round_trip(Output::AuthorizationApprovalStored(
        AuthorizationApprovalRecorded {
            authorization_request_slot: request_slot(),
            authorization_approval_decision: AuthorizationApprovalDecision::Approve,
        },
    ));
    round_trip(Output::ConfigurationRefused(ConfigurationRejected::new(
        ConfigurationRejectionReason::ManagerAuthorityRequired,
    )));
    round_trip(Output::OperationUnimplemented(RequestUnimplemented {
        operation_kind: OperationKind::Configure,
        unimplemented_reason: UnimplementedReason::DependencyNotReady,
    }));
    round_trip(Output::RootFoundingConfirmed(RootFoundingAccepted::new(
        root_anchor(),
        founding_signature(),
    )));
    round_trip(Output::RootFoundingRefused(RootFoundingRejected::new(
        RootFoundingRejectionReason::CohortMismatch,
    )));
    round_trip(Output::RootFoundingObserved(root_founding_status()));
}

//! The contract's own witness: every canonical value survives the wire, and
//! every canonical Datom line is the codec's own text for one of them.

use meta_signal_criome::{
    AuthorizationApproval, AuthorizationApprovalDecision, AuthorizationApprovalRecorded,
    ByteViewable, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    InterceptPolicyObservation, OperationKind, PendingFounding, Query, RequestUnimplemented,
    Response, Restorable, RootFoundingAcceptance, RootFoundingAccepted, RootFoundingInitiation,
    RootFoundingObservation, RootFoundingRejected, RootFoundingRejectionReason, RootFoundingState,
    RootFoundingStatus, Signal, Signalizable, UnimplementedReason,
};
use signal_criome::{
    AuthorizationMode, BlsPublicKey, Contract, ContractParent, CriomeDaemonConfiguration,
    FoundingMember, FoundingSignature, GenesisDomainTag, Identity, PrincipalName, ReplayNonce,
    RootGenesis, Rule, SignatureEnvelope, SignatureScheme,
};

fn genesis() -> RootGenesis {
    RootGenesis {
        contract: Contract {
            rule: Rule::SignedBy(Identity::Host(PrincipalName::from("prometheus"))),
            contract_parent: ContractParent::Root,
        },
        founding_member_vector: vec![FoundingMember {
            identity: Identity::Host(PrincipalName::from("prometheus")),
            bls_public_key: BlsPublicKey::from("b0b1"),
        }],
        genesis_domain_tag: GenesisDomainTag::CriomeRootFoundingV1,
        replay_nonce: ReplayNonce::from("nonce-1"),
    }
}

fn configuration() -> CriomeDaemonConfiguration {
    CriomeDaemonConfiguration {
        first_daemon_path: String::from("/run/criome/data.sock"),
        second_daemon_path: String::from("/run/criome/owner.sock"),
        daemon_path_option: None,
        bls_public_key_option: Some(BlsPublicKey::from("b0b1")),
        authorization_mode: AuthorizationMode::ClientApproval,
        identity_option: Some(Identity::Host(PrincipalName::from("prometheus"))),
        router_submission_configuration_option: None,
        quorum_window_nanos_option: Some(60_000_000_000),
    }
}

fn canonical_queries() -> Vec<Query> {
    vec![
        Query::ObserveRootFounding(RootFoundingObservation {}),
        Query::ObserveInterceptPolicies(InterceptPolicyObservation {}),
        Query::Configure(configuration()),
        Query::InitiateRootFounding(RootFoundingInitiation {
            root_genesis: genesis(),
        }),
        Query::AcceptRootFounding(RootFoundingAcceptance {
            root_anchor_digest: String::from("an(h0r"),
            root_genesis: genesis(),
        }),
        Query::SubmitAuthorizationApproval(AuthorizationApproval {
            authorization_request_slot: String::from("slot-1"),
            authorization_approval_decision: AuthorizationApprovalDecision::Approve,
        }),
        Query::RetractInterceptPolicyObservation(String::from("stream-1")),
    ]
}

fn canonical_responses() -> Vec<Response> {
    vec![
        Response::ConfigurationApplied(Configured {
            configuration_generation: 7,
        }),
        Response::ConfigurationRefused(ConfigurationRejected {
            configuration_rejection_reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        Response::OperationUnimplemented(RequestUnimplemented {
            operation_kind: OperationKind::AcceptRootFounding,
            unimplemented_reason: UnimplementedReason::NotBuiltYet,
        }),
        Response::RootFoundingRefused(RootFoundingRejected {
            root_founding_rejection_reason: RootFoundingRejectionReason::AlreadyFounded,
        }),
        Response::RootFoundingObserved(RootFoundingStatus {
            root_founding_state: RootFoundingState::Gathering,
            pending_founding_vector: vec![PendingFounding {
                root_anchor_digest: String::from("an(h0r"),
                root_genesis: genesis(),
                identity: Identity::Cluster(PrincipalName::from("cluster")),
            }],
        }),
        Response::RootFoundingConfirmed(RootFoundingAccepted {
            root_anchor_digest: String::from("an(h0r"),
            founding_signature: FoundingSignature {
                identity: Identity::Host(PrincipalName::from("prometheus")),
                signature_envelope: SignatureEnvelope {
                    signature_scheme: SignatureScheme::Bls12_381MinPk,
                    bls_public_key: BlsPublicKey::from("b0b1"),
                    bls_signature: String::from("51g1"),
                },
            },
        }),
        Response::AuthorizationApprovalStored(AuthorizationApprovalRecorded {
            authorization_request_slot: String::from("slot-1"),
            authorization_approval_decision: AuthorizationApprovalDecision::Defer,
        }),
    ]
}

#[test]
fn every_canonical_query_restores_from_fresh_peer_bytes() {
    for query in canonical_queries() {
        let received = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), query);
    }
}

#[test]
fn every_canonical_response_restores_from_fresh_peer_bytes() {
    for response in canonical_responses() {
        let received =
            Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), response);
    }
}

#[test]
fn a_malformed_archive_is_refused() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
    assert!(Signal::<Response>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
mod canonical {
    use super::{canonical_queries, canonical_responses};
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_criome::{Query, Response};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    const CANONICAL: &str = include_str!("../examples/canonical.datom");

    fn budget() -> Budget {
        Budget {
            remaining: 1 << 20,
            reader: ReaderBudget { remaining: 1 << 20 },
            depth: 0,
            maximum_depth: 256,
        }
    }

    fn lines() -> Vec<String> {
        CANONICAL
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .map(str::to_string)
            .collect()
    }

    /// Rewrite `examples/canonical.datom` from the canonical values. The file
    /// is the codec's product, never text spelled by hand; run this whenever
    /// the canonical values change:
    ///
    /// ```text
    /// cargo test --features datom -- --ignored rewrite_the_canonical_file
    /// ```
    #[test]
    #[ignore = "writes the source tree; run deliberately when the values change"]
    fn rewrite_the_canonical_file() {
        let mut out = String::from("; Canonical Datom examples for meta-signal-criome.\n");
        out.push_str("; Written by `rewrite_the_canonical_file`; never spelled by hand.\n\n");
        for query in canonical_queries() {
            out.push_str(&query.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        for response in canonical_responses() {
            out.push_str(&response.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/canonical.datom"),
            out,
        )
        .expect("write canonical");
    }

    /// The canonical file is the codec's own text for the canonical values.
    #[test]
    fn the_canonical_file_is_what_the_codec_writes() {
        let mut written: Vec<String> = Vec::new();
        for query in canonical_queries() {
            written.push(query.datomize(vec![]).protosize().textualize());
        }
        for response in canonical_responses() {
            written.push(response.datomize(vec![]).protosize().textualize());
        }
        assert_eq!(lines(), written);
    }

    /// Every line the file carries actualizes — as a request or as a reply,
    /// never as neither and never as both.
    #[test]
    fn every_canonical_line_actualizes_into_exactly_one_root() {
        let lines = lines();
        assert!(!lines.is_empty());
        for line in lines {
            let query = Potential::<Query>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            let response = Potential::<Response>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            match (query, response) {
                (Some(query), None) => {
                    assert_eq!(query.datomize(vec![]).protosize().textualize(), line)
                }
                (None, Some(response)) => {
                    assert_eq!(response.datomize(vec![]).protosize().textualize(), line)
                }
                (Some(_), Some(_)) => panic!("ambiguous canonical line: {line}"),
                (None, None) => panic!("unreadable canonical line: {line}"),
            }
        }
    }
}

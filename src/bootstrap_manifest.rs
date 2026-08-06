//! Explicit producer-owned bootstrap authority state for the owner Criome Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    96, 215, 64, 173, 220, 51, 15, 176, 103, 165, 183, 117, 115, 193, 148, 204, 252, 234, 229, 249,
    5, 79, 81, 189, 51, 204, 98, 54, 209, 46, 84, 230,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 42266;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 39515;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 31052, 0x65069b82b20c8eae);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 16146, 0x7ea55d29d7ec2a24);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 25033, 0x1e197948b099912e);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 30653, 0x373288b85d0c5da4);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 65334, 0xc05bcad1625903b0);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 51582, 0xe22fabaed8ab5ef0);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 42133, 0x21f81a80b77fbc66);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 18595, 0x685f279d9caaed3c);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 16775, 0x7de5806680b473fc);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 42160, 0x9ac743c7c3f3a593);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 36102, 0x14b3b2ee1a079e9c);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 39335, 0xa6e65670366077c6);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 8779, 0xf0f2d6460a282ad8);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 43354, 0xe9b113696206ac58);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 30954, 0x140ca42e6b862883);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 2585, 0xa604578cd9e5d492);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    65291, 58135, 49334, 47549, 24290, 44748, 37934, 53244, 4904, 425,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "OwnerRequest", 415, 0x24cdaebb31185be9),
    DeclarationSeat::new(Some(415), "Configure", 43734, 0x4c892cd1ec43f93b),
    DeclarationSeat::new(
        Some(415),
        "CreateInterceptPolicy",
        43261,
        0x981215b876e1b3f3,
    ),
    DeclarationSeat::new(
        Some(415),
        "ReplaceInterceptPolicy",
        27715,
        0x84f4913df62df2a0,
    ),
    DeclarationSeat::new(Some(415), "CancelInterceptPolicy", 8592, 0x26ff458db14768c3),
    DeclarationSeat::new(
        Some(415),
        "ListInterceptPolicies",
        38233,
        0xf25a8a4f26aae628,
    ),
    DeclarationSeat::new(
        Some(415),
        "ObserveInterceptPolicies",
        39356,
        0x17d9273be5669d6d,
    ),
    DeclarationSeat::new(
        Some(415),
        "RetractInterceptPolicyObservation",
        32259,
        0xacafa817d1662259,
    ),
    DeclarationSeat::new(Some(415), "FetchParkedRequests", 5878, 0xcfe282dc49f49784),
    DeclarationSeat::new(Some(415), "AnswerParkedRequest", 56186, 0x0eb4153059e77784),
    DeclarationSeat::new(
        Some(415),
        "ObserveParkedAuthorizations",
        43591,
        0x70ff6a1e3cbd4939,
    ),
    DeclarationSeat::new(
        Some(415),
        "SubmitAuthorizationApproval",
        45552,
        0x261711204057ce97,
    ),
    DeclarationSeat::new(Some(415), "AcceptRootFounding", 50734, 0xbc81421473a2e232),
    DeclarationSeat::new(Some(415), "InitiateRootFounding", 52923, 0x915fad46c0bbfa99),
    DeclarationSeat::new(Some(415), "ObserveRootFounding", 17842, 0x0cea4f3256c943a3),
    DeclarationSeat::new(None, "OwnerReply", 28586, 0xd3ef811ab0bd2b1e),
    DeclarationSeat::new(
        Some(28586),
        "ConfigurationApplied",
        5835,
        0x11d90be48ff3df6c,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPolicyCreated",
        42448,
        0x7a1d9a161e5e6f94,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPolicyReplaced",
        38436,
        0x3f12c3107b8e7f36,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPolicyCancelled",
        12445,
        0x2a62aaacd8f70714,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPoliciesListed",
        21220,
        0x3de16b9c98909677,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPolicyObservationOpened",
        24910,
        0x5a3a750ff35797ea,
    ),
    DeclarationSeat::new(
        Some(28586),
        "InterceptPolicyObservationRetracted",
        58910,
        0x345bb6e9c29caba8,
    ),
    DeclarationSeat::new(
        Some(28586),
        "ParkedRequestsFetched",
        63590,
        0x5f47d12f041273e3,
    ),
    DeclarationSeat::new(
        Some(28586),
        "ParkedRequestAnswered",
        15271,
        0x6ebb79d0ae7cba5e,
    ),
    DeclarationSeat::new(
        Some(28586),
        "ParkedAuthorizations",
        11066,
        0x9910ee4b3d6db874,
    ),
    DeclarationSeat::new(
        Some(28586),
        "AuthorizationApprovalStored",
        21261,
        0x19bf7e7a8aa13eed,
    ),
    DeclarationSeat::new(Some(28586), "ConfigurationRefused", 158, 0x9edf94585db6e07b),
    DeclarationSeat::new(
        Some(28586),
        "OperationUnimplemented",
        35119,
        0x1b2100079b693739,
    ),
    DeclarationSeat::new(
        Some(28586),
        "RootFoundingConfirmed",
        43565,
        0xa9e07d16351c58b3,
    ),
    DeclarationSeat::new(
        Some(28586),
        "RootFoundingRefused",
        52305,
        0x1152542700fb8e29,
    ),
    DeclarationSeat::new(
        Some(28586),
        "RootFoundingObserved",
        6713,
        0xb13f4189ad67214a,
    ),
    DeclarationSeat::new(None, "ConfigurationGeneration", 15541, 0x04b0202433c4f129),
    DeclarationSeat::new(None, "Configured", 43754, 0x578d481629a7081d),
    DeclarationSeat::new(
        None,
        "ConfigurationRejectionReason",
        45535,
        0xc4cb3e3e65374d27,
    ),
    DeclarationSeat::new(
        Some(45535),
        "ManagerAuthorityRequired",
        58836,
        0x2cfc7e7858a08a73,
    ),
    DeclarationSeat::new(
        Some(45535),
        "MalformedConfiguration",
        24403,
        0xbf6ba44ab78ec4da,
    ),
    DeclarationSeat::new(Some(45535), "StoreUnavailable", 5273, 0x9e75b2d2044116e1),
    DeclarationSeat::new(None, "ConfigurationRejected", 40645, 0x7d980dcb71f605a8),
    DeclarationSeat::new(
        None,
        "AuthorizationApprovalDecision",
        9747,
        0x5fdfa435448085c7,
    ),
    DeclarationSeat::new(Some(9747), "Approve", 43622, 0x5716ee50c65621e7),
    DeclarationSeat::new(Some(9747), "Reject", 64441, 0x3cff2a193457c150),
    DeclarationSeat::new(Some(9747), "Defer", 55953, 0x2c932b4ee8307cbb),
    DeclarationSeat::new(None, "AuthorizationApproval", 36451, 0x640e8dd0b3e4d462),
    DeclarationSeat::new(
        None,
        "AuthorizationApprovalRecorded",
        28691,
        0xaec6f40a258df98a,
    ),
    DeclarationSeat::new(None, "RootFoundingAcceptance", 23437, 0x38e891dba66cef8e),
    DeclarationSeat::new(
        None,
        "RootFoundingRejectionReason",
        22203,
        0x206131dcb3edd35f,
    ),
    DeclarationSeat::new(Some(22203), "CohortMismatch", 45258, 0xa9831a63eca6ef0c),
    DeclarationSeat::new(Some(22203), "AlreadyFounded", 9592, 0xed50e2a1d72efd19),
    DeclarationSeat::new(
        Some(22203),
        "ManagerAuthorityRequired",
        37256,
        0x783020a7dfe3d50d,
    ),
    DeclarationSeat::new(Some(22203), "MalformedGenesis", 61286, 0x59fa2b4ef287d64a),
    DeclarationSeat::new(None, "RootFoundingAccepted", 39033, 0x7ff22d5d83fb7b80),
    DeclarationSeat::new(None, "RootFoundingRejected", 25667, 0xf046c343396397d1),
    DeclarationSeat::new(None, "RootFoundingInitiation", 4898, 0x2e7a3718a65c5d4e),
    DeclarationSeat::new(None, "RootFoundingObservation", 37746, 0xaf83e139ade89d37),
    DeclarationSeat::new(None, "RootFoundingState", 61332, 0x0e8e0ff5f2aee656),
    DeclarationSeat::new(Some(61332), "Unfounded", 25853, 0xb7974df0f7cc90eb),
    DeclarationSeat::new(Some(61332), "Gathering", 4539, 0x0f8309626a2b0f8e),
    DeclarationSeat::new(Some(61332), "Founded", 7410, 0x6dd32d32320550bc),
    DeclarationSeat::new(None, "PendingFounding", 11238, 0x7fcdf38c18ee5005),
    DeclarationSeat::new(None, "RootFoundingStatus", 42972, 0x16c4ef92089e376c),
    DeclarationSeat::new(
        None,
        "InterceptPolicyObservation",
        43428,
        0x118714766624902c,
    ),
    DeclarationSeat::new(None, "InterceptPolicyChange", 18187, 0xdc1a1e4f799a62d9),
    DeclarationSeat::new(Some(18187), "Created", 17825, 0xaa0c04ba6f2cc137),
    DeclarationSeat::new(Some(18187), "Replaced", 48214, 0xb6d952267f758ae5),
    DeclarationSeat::new(Some(18187), "Cancelled", 3784, 0xe725683e3b38b930),
    DeclarationSeat::new(
        None,
        "InterceptPolicyStreamToken",
        41909,
        0x455b2ad308a94970,
    ),
    DeclarationSeat::new(None, "OperationKind", 32715, 0xefee3bb49b89b426),
    DeclarationSeat::new(Some(32715), "Configure", 7411, 0x0296b28d375c799f),
    DeclarationSeat::new(
        Some(32715),
        "CreateInterceptPolicy",
        23378,
        0x2ed1f96777eff1c4,
    ),
    DeclarationSeat::new(
        Some(32715),
        "ReplaceInterceptPolicy",
        4194,
        0x4b717ab45b12767f,
    ),
    DeclarationSeat::new(
        Some(32715),
        "CancelInterceptPolicy",
        53203,
        0x6f4d65d6972ac520,
    ),
    DeclarationSeat::new(
        Some(32715),
        "ListInterceptPolicies",
        25649,
        0xe7d7d9c8dbcdddc4,
    ),
    DeclarationSeat::new(
        Some(32715),
        "ObserveInterceptPolicies",
        2613,
        0x7571411ad2662343,
    ),
    DeclarationSeat::new(
        Some(32715),
        "RetractInterceptPolicyObservation",
        51496,
        0xba41e094edb0ad1e,
    ),
    DeclarationSeat::new(
        Some(32715),
        "FetchParkedRequests",
        17753,
        0x5597c45d36b52ad3,
    ),
    DeclarationSeat::new(Some(32715), "AnswerParkedRequest", 5143, 0x7cd4181921b35cc2),
    DeclarationSeat::new(
        Some(32715),
        "ObserveParkedAuthorizations",
        1927,
        0xd19a2409db0387c1,
    ),
    DeclarationSeat::new(
        Some(32715),
        "SubmitAuthorizationApproval",
        56070,
        0xaebe51b73a5f8e9f,
    ),
    DeclarationSeat::new(Some(32715), "AcceptRootFounding", 2439, 0x9edbe4148565056b),
    DeclarationSeat::new(
        Some(32715),
        "InitiateRootFounding",
        62392,
        0x415b6e9a71e634fd,
    ),
    DeclarationSeat::new(
        Some(32715),
        "ObserveRootFounding",
        64207,
        0x917596c1f9152a8c,
    ),
    DeclarationSeat::new(None, "UnimplementedReason", 33628, 0x65c8475f99e508b5),
    DeclarationSeat::new(Some(33628), "NotBuiltYet", 41774, 0xf8a935d12eb2a7cc),
    DeclarationSeat::new(Some(33628), "DependencyNotReady", 43756, 0x08e40333b96cad3d),
    DeclarationSeat::new(None, "RequestUnimplemented", 17679, 0x25a3f3e2dccdf740),
];

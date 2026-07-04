# meta-signal-criome - architecture

`meta-signal-criome` is the meta-only wire contract for privileged
`criome-daemon` configuration. It is the authority/configuration companion to
the ordinary `signal-criome` contract.

## Role

The crate defines the meta channel that lets the owning authority configure a
criome daemon. The baseline operation is `Configure`, carrying
`signal_criome::CriomeDaemonConfiguration`, the same typed startup record the
daemon decodes from its binary startup file.

## Direction

`meta-signal-criome` is the second leg of the criome contract pair. Every
Persona component has exactly two contracts: the ordinary `signal-<component>`
and the meta `meta-signal-<component>`. This repo completes that pair for
`criome` by giving it the meta authority surface its ordinary contract lacked,
so the `criome-daemon` has the configuration plane its startup record already
implies.

Possession of this meta socket is the MVP authority boundary for intercept
policy mutation and parked-request answers. The same owner socket that applies
daemon configuration also carries policy create/replace/cancel/list/observe,
parked-request fetch, and parked-request answer; holding the socket is what
distinguishes the authority caller from an ordinary peer.

## Owned

- Meta authority wire vocabulary for criome.
- The `Configure(CriomeDaemonConfiguration)` operation.
- Intercept-policy owner operations:
  `CreateInterceptPolicy`, `ReplaceInterceptPolicy`, `CancelInterceptPolicy`,
  `ListInterceptPolicies`, `ObserveInterceptPolicies`, and
  `RetractInterceptPolicyObservation`.
- Parked Spirit request control operations:
  `FetchParkedRequests` and `AnswerParkedRequest`.
- The owner-only root-founding accept: `AcceptRootFounding(RootFoundingAcceptance)`
  — the explicit owner action that founds this node's root (there is no
  auto-approval). The acceptance carries the self-certifying `RootAnchorDigest`
  plus the full `RootGenesis` cohort (imported from `signal-criome`) so the daemon
  re-derives and matches the anchor before its master key emits an attached
  founding signature. Replies: `RootFoundingAccepted` (the anchor + this node's
  attached, scheme-tagged `FoundingSignature`) and `RootFoundingRejected`
  (`CohortMismatch | AlreadyFounded | ManagerAuthorityRequired | MalformedGenesis`).
- Configuration replies: `Configured`, `ConfigurationRejected`, and
  `RequestUnimplemented`.
- Optional NOTA projection behind the `nota-text` feature.

## Not Owned

- Ordinary criome trust traffic: sign, verify, identity, attestation, and
  authorization operations live in `signal-criome`.
- The shared intercept-policy and parked-request records live in
  `signal-criome`; this contract imports them and exposes the owner/meta
  authority verbs.
- Criome daemon state, sockets, actors, and storage live in `criome`.
- Schema generation machinery lives in `schema-next` / `schema-rust-next`.

## Code Map

- `schema/lib.schema` is the source of the meta wire vocabulary.
- `build.rs` runs `schema-rust-next` and imports
  `signal_criome::CriomeDaemonConfiguration` from `signal-criome`.
- `src/schema/lib.rs` is the checked-in generated artifact.
- `src/lib.rs` re-exports the generated nouns and keeps only tiny
  handwritten accessors.
- `Cargo.toml` keeps `nota-text` optional and pins the rkyv feature set.
- `flake.nix` builds, tests, formats, documents, and lints the contract in both
  no-feature and `nota-text` modes.

## Invariants

- The crate is wire-only: no daemon runtime, no actors, no storage, no tokio.
- Default builds are NOTA-free.
- The meta contract reuses `signal_criome::CriomeDaemonConfiguration`; it does
  not mirror the daemon configuration record.
- The meta contract reuses `signal_criome` intercept-policy and parked-request
  records; criome remains the policy-state owner.
- The implementation is schema-derived `WireContract`; no handwritten
  `signal_frame::signal_channel!` remains.

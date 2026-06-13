# meta-signal-criome - architecture

`meta-signal-criome` is the meta-only wire contract for privileged
`criome-daemon` configuration. It is the authority/configuration companion to
the ordinary `signal-criome` contract.

## Role

The crate defines the meta channel that lets the owning authority configure a
criome daemon. The baseline operation is `Configure`, carrying
`signal_criome::CriomeDaemonConfiguration`, the same typed startup record the
daemon decodes from its binary startup file.

## Owned

- Meta authority wire vocabulary for criome.
- The `Configure(CriomeDaemonConfiguration)` operation.
- Configuration replies: `Configured`, `ConfigurationRejected`, and
  `RequestUnimplemented`.
- Optional NOTA projection behind the `nota-text` feature.

## Not Owned

- Ordinary criome trust traffic: sign, verify, identity, attestation, and
  authorization operations live in `signal-criome`.
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
- The implementation is schema-derived `WireContract`; no handwritten
  `signal_frame::signal_channel!` remains.

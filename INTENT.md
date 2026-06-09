# INTENT — meta-signal-criome

*The meta-only wire contract for privileged `criome` daemon configuration.
Companion to `Cargo.toml` and the ordinary `signal-criome` contract.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is for the `meta-signal-criome`
contract. Workspace-shape intent stays in `primary/INTENT.md`; the component
daemon intent stays in `criome/INTENT.md`; ordinary criome trust traffic
(sign, verify, identity, attestation, authorization) stays in
`signal-criome/INTENT.md`.

## Why this repo exists

Every Persona component has exactly two contracts: `signal-<component>`
(ordinary) and `meta-signal-<component>` (meta). `meta-signal-criome` is the
second leg for `criome` — the authority surface that configures the
`criome-daemon`. `criome/INTENT.md` already names this contract as its target
meta pair. Before this repo, `criome` had only its ordinary contract; this
completes the pair.

## The channel shape

The meta plane's baseline content is daemon configuration. The channel carries
a single `Configure` operation whose payload is `CriomeDaemonConfiguration`
(the daemon's socket and `criome.sema` store location). This mirrors the
daemon's binary startup record; later reconfiguration arrives over this meta
plane as the same typed record, never as flags.

- **Request:** `Configure(CriomeDaemonConfiguration)`.
- **Replies:** `Configured`, `ConfigurationRejected` (typed reason),
  `RequestUnimplemented`.

The root-of-trust material that "Criome verifies; Persona decides" rests on is
daemon configuration and so extends the `Configure` payload rather than
appearing as bespoke operations.

## Pending reconciliation

`CriomeDaemonConfiguration` is defined here as the canonical contract home. The
`criome` daemon currently defines an equivalent shape locally
(`criome/src/daemon.rs`); it should adopt this type so startup decoding and
meta reconfiguration share one record. Pre-production reconciliation — no
backward-compatibility constraint.

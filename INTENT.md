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
`Configure(CriomeDaemonConfiguration)`, using the same record the criome daemon
decodes at binary startup, so startup and meta reconfiguration share one
definition that lives in the ordinary `signal-criome` contract.

The same owner socket now carries the intercept-policy MVP authority surface:
create, replace, cancel, list, and observe criome-owned intercept policies;
fetch parked Spirit requests; and answer a parked request. Those payload records
are imported from `signal-criome` rather than mirrored here, because criome owns
policy state and parked request identity. Possession of this meta socket is the
MVP authority boundary for policy mutation and parked-request answers.

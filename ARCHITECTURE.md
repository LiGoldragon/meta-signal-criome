# meta-signal-criome architecture

## Center

This repository owns the owner Criome Interface. It is the privileged relation
between a Criome daemon and the authority permitted to configure it, mutate
intercept policy, answer parked work, approve authorization slots, and conduct
root founding.

The Interface is independent of Rust, the current daemon, and the current
operating system. Rust is one verified bootstrap projection. Beauty and elegant,
extendable logic win: shared ordinary identities are imported, never mirrored.

## Authority and imports

The ethos/interface.ethos file is a strict, role-free Interface transaction.
Local authority, declaration, variant, and canonical-order seats live explicitly
in src/bootstrap_manifest.rs. No identity is derived from spelling, order, or
content hashes.

The ordinary producer is pinned exactly at signal-criome commit
9436a3b8ffc2ee508ee1aaec807f5fe293187d59. During every build, build.rs:

1. locates the producer-owned Ethos directory through Cargo metadata;
2. proves the located source equals the source compiled into that producer;
3. admits the selected producer declaration seats into the owner authority;
4. applies the owner Interface transaction;
5. verifies the encoded Rust projection byte-for-byte; and
6. publishes this repository's owned Ethos directory.

The generated projection contains encoded local and imported coordinates only.
There is no legacy schema source, readable generated layer, or readable alias
layer.

## Current bootstrap boundary

The src/schema/lib/behavior.rs file supplies behavior not yet expressible in the
strict Interface:

- producer-shared WireShape, WireValue, and rkyv structural behavior;
- optional Dotos behavior;
- OwnerRequest and OwnerReply role seating;
- Signal framing under allocated contract ID 4, wire revision 2.

The shared representation is owned by signal-criome so imported types and local
types inhabit one structural wire algebra. When Protos expresses these behavior
forms, the handwritten layer should shrink without changing the Interface.

## Domain boundary

The owner Interface carries:

- daemon configuration;
- intercept-policy create, replace, cancel, list, observe, and retract;
- parked Spirit request fetch and answer;
- parked authorization observation and authorization approval by durable slot;
- root-founding initiation, observation, explicit acceptance, and typed
  acceptance/refusal/status replies.

Ordinary sign, verify, identity, attestation, and authorization traffic belongs
to signal-criome. Daemon execution, persistence, actors, sockets, private keys,
and policy evaluation belong elsewhere.

## Evidence

- tests/interface_contract.rs proves the owner source imports the producer and
  that generated local/imported coordinates are encoded.
- tests/frame.rs proves both owner roles round-trip through binding 4/revision 2.
- tests/round_trip.rs and tests/canonical_examples.rs prove human Dotos heads
  survive the encoded Rust projection.
- tests/dependency_boundary.rs proves exact producer/generator pins and keeps
  bootstrap and retired crates out of the runtime graph.

Structural changes begin in Ethos, preserve existing seats, mint explicit seats
only for new identities, regenerate once, and renew all evidence.

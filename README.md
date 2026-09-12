# meta-signal-criome

The meta Signal contract for Criome: privileged daemon control. Configuration,
root founding, authorization approval, and intercept-policy administration.

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates the
projection with `ethos-zero` and asserts it against the committed
`src/generated/signal.rs`, so the two can never drift.

Seventeen names come from `signal-criome` — the genesis, the daemon
configuration, the intercept-policy shapes, the parked-request shapes. The
evaluated authorization surface belongs to the ordinary contract; this one
carries only the owner's control of it.

`examples/canonical.datom` holds one canonical Datom value per line, written
by the codec and never spelled by hand; `tests/contract.rs` asserts every line
is exactly what the codec writes and that each actualizes back into exactly
one of `Query` and `Response`.

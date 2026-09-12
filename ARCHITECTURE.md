# meta-signal-criome architecture

## Center

This repository owns the privileged Criome surface: what a manager may set and
what it is told in return. Configuration, root founding, authorization
approval, intercept-policy administration.

It owns no policy evaluation, no key custody, no storage, and no daemon.

## Authority and projection

`ethos/signal.ethos` is the sole textual source — a `Signal` root holding the
import list, the request variants, the reply variants, and the type
declarations. `ethos-zero` projects it into `src/generated/signal.rs`, which
is committed; `build.rs` generates afresh and asserts equality.

`src/lib.rs` re-exports the projection, and re-exports the portable frame from
`signal` so a consumer speaks this contract without naming `signal` itself.
The request and reply roots are named `Query` and `Response` — `ethos-zero`
names them, not this contract.

## The ordinary contract is imported, not restated

`RootGenesis`, `CriomeDaemonConfiguration`, `Identity`, `FoundingSignature`,
`InterceptPolicy` and its proposal, cancellation and identifier,
`ParkedRequestQuery`, `ParkedRequestAnswer`, `ParkedRequestSnapshot`,
`ParkedRequestResolution`, `ParkedAuthorizationObservation`,
`ParkedAuthorizationSnapshot`, `ActiveInterceptPolicies`,
`AuthorizationRequestSlot` and `RootAnchorDigest` all come from
`signal-criome`.

The owner does not speak a different Criome from everyone else. It speaks the
same objects with more authority, so the objects are the same types. What this
contract adds is only the *control* vocabulary that has no ordinary
counterpart: approvals, configuration generations, founding state and the
typed refusals for each.

## The `links` key is gone

This repository declared `links = "meta-signal-criome"`. Cargo admits exactly
one package per `links` key in a dependency graph and refuses at *resolution*,
masking every compile error behind it — and `criome`, `mentci` and
`mentci-lib` all pin this crate, so the three could never be repinned
separately. Nothing here needs a native library, so the key is deleted.

It is worth recording why the key bit so hard: cargo source identity is the
pin *string*, not the commit. `.../meta-signal-criome` and
`.../meta-signal-criome.git` name one commit and two packages, and a `links`
key refuses the pair. `criome` and `mentci-lib` spelled the `.git` form while
`mentci` spelled the bare one, which is a resolution failure waiting on the
first graph that holds all three.

## The producer cut this contract is generated against

`ethos-zero` 9.0.0 `b232d35e`, whose projection derives
`datom_codec::Composing` from the split composing kind that `datom-codec`
0.27.0 `6dccc76b` reintroduced arity into; `signal` 5.0.0 `7bcb0949`,
generated against the same pair; `protos` 0.30.1 `171b21f6`; and
`signal-criome` 2.0.0 `5bfa61b5`, generated against all four.

These move together and are spelled identically everywhere, without a `.git`
suffix: cargo source identity is the pin string, not the commit, so one commit
under two spellings is two packages — and `signal` carries `links = "signal"`,
which admits exactly one per graph.

## Boundaries

`signal` for the frame and the framing, `signal-criome` for Criome's shapes,
`rkyv` for the archive, and under the optional `datom` feature `datom-codec`
and `protos`. Nothing else.

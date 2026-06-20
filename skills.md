# meta-signal-criome agent guide

Read, in order, before changing this repository:

1. `INTENT.md`
2. `ARCHITECTURE.md`
3. `/home/li/primary/skills/contract-repo.md`
4. `/home/li/primary/skills/component-triad.md`
5. `/home/li/primary/skills/rust-discipline.md`
6. `/home/li/primary/skills/nix-discipline.md`

This crate is a wire-only meta signal contract. It has no daemon runtime,
actor system, storage, sockets, signing implementation, or Tokio dependency.

Shared daemon types live in `signal-criome` and are imported here instead of
duplicated. The meta plane currently owns daemon configuration, parked
authorization observation, and authorization approval by
`AuthorizationRequestSlot`.

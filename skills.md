# Working in meta-signal-criome

Read ARCHITECTURE.md first.

## Structural law

- ethos/interface.ethos is the sole readable structure.
- src/bootstrap_manifest.rs owns explicit local seats.
- signal-criome owns every imported ordinary seat.
- Never derive identity from spelling, position, or hashes.
- Never add a legacy schema source or readable Rust aliases.
- Never hand-edit generated.rs.

## Change sequence

1. Change the strict Ethos transaction.
2. Preserve existing seats and mint explicit seats only for new identities.
3. Run META_SIGNAL_CRIOME_UPDATE_INTERFACE_ARTIFACTS=1 cargo check --lib.
4. Change encoded-name behavior only where the new structure requires it.
5. Renew Interface, frame, Dotos, and dependency-boundary witnesses.
6. Run formatting, default and all-feature tests, clippy with denied warnings,
   rustdoc with denied warnings, and nix flake check.

Publish the ordinary producer first whenever an imported identity changes, then
pin its exact commit here.

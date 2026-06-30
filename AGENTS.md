You MUST read `/home/li/primary/repos/lore/AGENTS.md` - the canonical agent
contract.

Repo role: meta-only wire contract for privileged `criome` daemon
configuration.

Required local reading:

- `ARCHITECTURE.md` first.
- `/home/li/primary/skills/contract-repo.md`.
- `/home/li/primary/skills/component-triad.md`.

Contract discipline:

- Keep default builds NOTA-free; gate human text projection behind
  `nota-text`.
- Keep this crate wire-only: no daemon runtime, no actors, no storage, no
  tokio.
- Ordinary trust traffic stays in `signal-criome`; this crate carries only
  meta authority/configuration vocabulary.

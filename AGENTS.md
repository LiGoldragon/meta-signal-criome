Repo role: meta-only wire contract for privileged `criome` daemon
configuration.

Required local reading:

- `ARCHITECTURE.md` first.

Contract discipline:

- Keep default builds NOTA-free; gate human text projection behind
  `nota-text`.
- Keep this crate wire-only: no daemon runtime, no actors, no storage, no
  tokio.
- Ordinary trust traffic stays in `signal-criome`; this crate carries only
  meta authority/configuration vocabulary.

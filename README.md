# meta-signal-criome

Meta signal contract for privileged criome daemon configuration.

The meta-only wire contract for `criome` — the second leg of the two-contract
pair (`signal-criome` ordinary + `meta-signal-criome` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `signal_criome::CriomeDaemonConfiguration` (socket + `criome.sema`
store location — the same record the daemon decodes at binary startup), with
`Configured` / `ConfigurationRejected` / `RequestUnimplemented` replies.

Default builds carry `nota-text` for CLI/debug projection; the wire is
binary/rkyv. See `INTENT.md`.

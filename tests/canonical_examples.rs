//! Canonical NOTA examples round-trip witness.

use meta_signal_criome::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    CriomeDaemonConfiguration, Input, OperationKind, Output, RequestUnimplemented,
    UnimplementedReason,
};
use nota_next::{NotaDecode, NotaEncode, NotaSource};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn configuration() -> CriomeDaemonConfiguration {
    CriomeDaemonConfiguration::new("/run/criome/criome.sock", "/var/lib/criome/criome.sema")
}

fn round_trip<Value>(value: Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let decoded = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(decoded, value, "decode for {text}");
    assert!(
        CANONICAL.contains(&text),
        "examples/canonical.nota missing line: {text}",
    );
}

#[test]
fn canonical_input_examples_round_trip() {
    round_trip(Input::Configure(configuration()));
}

#[test]
fn canonical_output_examples_round_trip() {
    round_trip(Output::Configured(ConfigurationGeneration::new(7).into()));
    round_trip(Output::ConfigurationRejected(ConfigurationRejected::new(
        ConfigurationRejectionReason::ManagerAuthorityRequired,
    )));
    round_trip(Output::RequestUnimplemented(RequestUnimplemented {
        operation: OperationKind::Configure,
        reason: UnimplementedReason::DependencyNotReady,
    }));
}

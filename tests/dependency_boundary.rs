use std::process::Command;

#[test]
fn default_runtime_tree_excludes_bootstrap_and_retired_crates() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    for forbidden in [
        "core-ethos",
        "name-table",
        "nota",
        "nota-codec",
        "rust-logos",
        "schema-language",
        "schema-rust",
        "sema-translator",
        "signal-core",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "default runtime tree must not contain {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn lockfile_has_one_exact_generator_and_ordinary_interface() {
    const LOCKFILE: &str = include_str!("../Cargo.lock");

    assert_eq!(LOCKFILE.matches("name = \"schema-rust\"").count(), 1);
    assert!(LOCKFILE.contains(
        "schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c85bd69357e9042729ba2df0052799756"
    ));
    assert_eq!(LOCKFILE.matches("name = \"signal-criome\"").count(), 1);
    assert!(LOCKFILE.contains(
        "signal-criome.git?rev=9436a3b8ffc2ee508ee1aaec807f5fe293187d59#9436a3b8ffc2ee508ee1aaec807f5fe293187d59"
    ));
    assert!(!LOCKFILE.contains("name = \"schema-language\""));
}

#[test]
fn dotos_text_is_the_only_text_projection_opt_in() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "dotos-text",
        ])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    assert!(tree.contains("dotos"), "Dotos must be opt-in:\n{tree}");
    for forbidden in ["nota-codec", "schema-language", "signal-core"] {
        assert!(
            !tree.contains(forbidden),
            "Dotos tree must not contain {forbidden}:\n{tree}"
        );
    }
}

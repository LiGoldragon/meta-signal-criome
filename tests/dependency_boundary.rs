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
fn lockfile_has_one_ordinary_interface_and_the_matching_generator() {
    const LOCKFILE: &str = include_str!("../Cargo.lock");

    // One ordinary producer Interface. Two signal-criome revisions would split
    // the wire types this owner Interface re-exports.
    assert_eq!(LOCKFILE.matches("name = \"signal-criome\"").count(), 1);
    assert!(LOCKFILE.contains(
        "signal-criome.git?rev=b85fe3408faa24b8439a3685396430a68bfbcb71#b85fe3408faa24b8439a3685396430a68bfbcb71"
    ));

    // This crate's own bootstrap generator is the revision build.rs is written
    // against. Sibling contract crates carry their own generator revisions in
    // their own build scripts; that duplication is build-only and is fenced by
    // default_runtime_tree_excludes_bootstrap_and_retired_crates.
    assert!(LOCKFILE.contains(
        "schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c85bd69357e9042729ba2df0052799756"
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

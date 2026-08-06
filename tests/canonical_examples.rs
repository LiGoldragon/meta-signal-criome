#![cfg(feature = "dotos-text")]

use dotos::{Document, DotosDecode, DotosEncode};
use meta_signal_criome::schema::lib::z2VKtS;

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

#[test]
fn canonical_owner_request_decodes_and_reencodes() {
    let document = Document::parse(CANONICAL).expect("canonical Dotos parses");
    assert_eq!(document.holds_root_objects(), 1);
    let request = z2VKtS::from_dotos_block(&document.root_objects()[0])
        .expect("canonical owner request decodes");
    assert_eq!(request.to_dotos(), "ObserveRootFounding.()");
}

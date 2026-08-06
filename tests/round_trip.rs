#![cfg(feature = "dotos-text")]

use dotos::{DotosEncode, DotosSource};
use meta_signal_criome::schema::lib::{z2VKtS, z2VWz5};

#[test]
fn owner_request_round_trips_through_dotos_without_readable_rust_aliases() {
    let request = z2VKtS::z2VR4u(z2VWz5 {});
    let text = request.to_dotos();
    assert_eq!(text, "ObserveRootFounding.()");
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VKtS>()
            .expect("Dotos decodes"),
        request,
    );
}

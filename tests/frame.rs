use meta_signal_criome::schema::lib::{
    ContractMarker, FrameBody, InputRoute, OutputRoute, z2VKtS, z2VQPE, z2VUG9, z2VWz5, z2VYmf,
};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(4),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

#[test]
fn handwritten_owner_roles_round_trip_encoded_values() {
    let request = z2VKtS::z2VR4u(z2VWz5 {});
    assert_eq!(request.route(), InputRoute::ObserveRootFounding);
    let encoded = request
        .encode_request_frame(exchange())
        .expect("request frame encodes");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded.route(), InputRoute::ObserveRootFounding);

    let reply = z2VUG9::z2VMVt(z2VYmf {
        field_0: z2VQPE::new(7),
    });
    assert_eq!(reply.route(), OutputRoute::ConfigurationApplied);
    let encoded = reply
        .clone()
        .encode_reply_frame(exchange())
        .expect("reply frame encodes");
    let decoded = ContractMarker::decode_frame(&encoded).expect("reply frame decodes");
    assert_eq!(
        decoded.into_body(),
        FrameBody::Reply {
            exchange: exchange(),
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(reply),
            )),
        }
    );
}

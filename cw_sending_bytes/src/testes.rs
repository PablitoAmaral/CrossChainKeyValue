#[cfg(test)]
mod tests {
    use crate::{contract, ibc, msg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_ibc_channel};
    use cosmwasm_std::{
        attr, Addr, Binary, IbcChannelCloseMsg, IbcChannelOpenMsg, IbcEndpoint, IbcOrder,
        IbcPacket, IbcPacketReceiveMsg, IbcTimeout, MessageInfo, Timestamp,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = MessageInfo {
            sender: Addr::unchecked("sender"),
            funds: vec![],
        };

        let msg = msg::InitMsg {
            initial_key: "magikarp".to_string(),
            initial_value: "gyarados".to_string(),
        };

        contract::instantiate(deps.as_mut(), env, info, msg).unwrap();

        let stored_value = contract::VALUES
            .load(&deps.storage, "magikarp".to_string())
            .unwrap();
        assert_eq!(stored_value, "gyarados".to_string());
    }
    #[test]
    fn test_ibc_channel_open() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let msg = IbcChannelOpenMsg::new_init(mock_ibc_channel(
            "My-channel",
            ibc::PROTOCOL_ORDERING,
            ibc::PROTOCOL_VERSION,
        ));
        let res = ibc::ibc_channel_open(deps.as_mut(), env.clone(), msg);
        assert!(
            res.is_ok(),
            "Channel open should succeed with correct protocol version and ordering"
        );

        let wrong_msg = IbcChannelOpenMsg::new_init(mock_ibc_channel(
            "My-channel",
            IbcOrder::Ordered,
            "not-the-vesion",
        ));
        let res = ibc::ibc_channel_open(deps.as_mut(), env.clone(), wrong_msg);
        assert!(
            res.is_err(),
            "Channel open should fail with incorrect protocol version"
        );
    }

    #[test]
    fn test_ibc_channel_close() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let msg = IbcChannelCloseMsg::new_init(mock_ibc_channel(
            "My-channel",
            ibc::PROTOCOL_ORDERING,
            ibc::PROTOCOL_VERSION,
        ));
        let res = ibc::ibc_channel_close(deps.as_mut(), env.clone(), msg);
        assert!(res.is_err(), "Channel close should return an error");
        let res = res.unwrap_err().to_string();
        assert_eq!(res, "Generic error: The game is infinite");
    }

    #[test]
    fn basic_test() {
        let str = String::from("hello");
        let bytes = str.into_bytes();
        let a = cosmwasm_std::Binary::new(bytes);
        assert_eq!(String::from_utf8(a.to_vec()).unwrap(), "hello");
    }

    #[test]
    fn test_ibc_packet_receive() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let key = "test_key".to_string();
        let expected_value = "test_value".to_string();

        contract::VALUES
            .save(&mut deps.storage, key.clone(), &expected_value)
            .unwrap();

        let packet = msg::Packet::Read(key.clone());
        let packet_data = packet.encode();

        let ibc_packet = IbcPacket::new(
            Binary::from(packet_data),
            IbcEndpoint {
                port_id: "src_port".to_string(),
                channel_id: "src_channel".to_string(),
            },
            IbcEndpoint {
                port_id: "dest_port".to_string(),
                channel_id: "dest_channel".to_string(),
            },
            1,
            IbcTimeout::with_timestamp(Timestamp::from_seconds(0)),
        );
        let msg = IbcPacketReceiveMsg::new(ibc_packet, Addr::unchecked("relayer_address"));

        let res = ibc::ibc_packet_receive(deps.as_mut(), env, msg).unwrap();

        assert_eq!(
            res.acknowledgement.unwrap(),
            Binary::from(expected_value.clone().into_bytes())
        );
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "received_packet"),
                attr("operation", "read"),
                attr("success", "true"),
            ]
        );
    }

    #[test]
    fn test_encode_decode_read() {
        let packet = msg::Packet::Read("my_key".to_string());

        let encoded = packet.encode();
        println!("Encoded Read (hex): {:x?}", encoded);

        let decoded = msg::Packet::decode(&encoded).unwrap();

        assert_eq!(decoded, msg::Packet::Read("my_key".to_string()));
        println!("Decode Read Test Passed");
    }

    #[test]
    fn test_encode_decode_write() {
        let packet = msg::Packet::Write("my_key".to_string(), "my_value".to_string());

        let encoded = packet.encode();
        println!("Encoded Write (hex): {:x?}", encoded);

        let decoded = msg::Packet::decode(&encoded).expect("Decoding failed");

        assert_eq!(
            decoded,
            msg::Packet::Write("my_key".to_string(), "my_value".to_string())
        );
        println!("Decode Write Test Passed");
    }
}

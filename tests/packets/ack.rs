const ACK_PACKET: [u8; 10] = [0xd4, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52];

#[test]
fn test_ack_packet() {
  // Receiver address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &ACK_PACKET,
    subtype: Some(FrameSubtype::Control(ControlSubtype::Ack)),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),
    duration_id: Some(DurationID::Duration(0)),

    receiver_address: "00:15:00:34:18:52".parse().unwrap(),

    ..Default::default()
  });
}

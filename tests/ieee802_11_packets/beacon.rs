const IEEE802_11_BEACON_PACKET: [u8; 110] = [
  0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x20, 0xf9, 0x88, 0xb1, 0xfc, 0x69, 0x02, 0x00, 0x00, 0x00,
  0x64, 0x00, 0x11, 0x04, 0x00, 0x09, 0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33, 0x01,
  0x08, 0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x03, 0x01, 0x0b, 0x05, 0x04, 0x00, 0x01,
  0x00, 0x00, 0x2a, 0x01, 0x04, 0x2f, 0x01, 0x04, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06,
  0x00, 0x10, 0x18, 0x01, 0x01, 0x00, 0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50,
  0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

#[test]
fn test_ieee802_11_beacon_packet() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Destination address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Source address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &IEEE802_11_BEACON_PACKET,
    subtype: FrameSubtype::Management(ManagementSubtype::Beacon),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(0),

    receiver_address: "FF:FF:FF:FF:FF:FF".parse().unwrap(),
    destination_address: Some("FF:FF:FF:FF:FF:FF".parse().unwrap()),

    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    station_address: None,

    fragment_number: Some(0),
    sequence_number: Some(3986),

    ssid: Some(vec![0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33]),
  });
}

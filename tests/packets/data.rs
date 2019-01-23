const DATA_PACKET: [u8; 131] = [
  0x08, 0x0a, 0x2c, 0x00, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x80, 0x1b, 0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00, 0x88, 0x8e,
  0x01, 0x03, 0x00, 0x5f, 0xfe, 0x00, 0x89, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0xf1, 0x9c, 0x5c, 0x80, 0x14, 0xf2, 0xfd, 0xf8, 0x75, 0xab, 0xfc, 0x0c, 0xcc, 0x23, 0x8a,
  0xe0, 0x27, 0x2b, 0x97, 0x11, 0x0c, 0x1e, 0x91, 0xd0, 0xb0, 0x7e, 0x5f, 0xbb, 0xfc, 0x87, 0x22,
  0xa9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00,
];

#[test]
fn test_data_packet() {
  // Receiver address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Transmitter address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Destination address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // STA address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)

  test_test_item(TestItem {
    bytes: &DATA_PACKET,
    subtype: Some(FrameSubtype::Data(DataSubtype::Data)),
    ds_status: DSStatus::FromDSToSTA,

    retry: true,

    duration_id: DurationID::Duration(44),

    receiver_address: "00:16:bc:3d:aa:57".parse().unwrap(),
    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    destination_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    station_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(440),

    ..Default::default()
  });
}
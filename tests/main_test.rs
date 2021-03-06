use ieee80211::*;

#[derive(Default)]
struct TestItem<'a> {
    bytes: &'a [u8],

    version: Option<FrameVersion>,

    type_: Option<FrameType>,
    subtype: Option<FrameSubtype>,
    ds_status: Option<DSStatus>,

    more_fragments: bool,
    retry: bool,
    pwr_mgt: bool,
    more_data: bool,
    protected: bool,
    order: bool,

    duration_id: Option<DurationID>,

    receiver_address: MacAddress,
    transmitter_address: Option<MacAddress>,

    destination_address: Option<MacAddress>,
    source_address: Option<MacAddress>,

    bssid_address: Option<MacAddress>,
    station_address: Option<MacAddress>,

    fragment_number: Option<u8>,
    sequence_number: Option<u16>,

    ssid: Option<Vec<u8>>,

    timestamp: Option<u64>,
    beacon_interval: Option<f64>,
    capabilities_info: Option<CapabilitiesInfo>,

    supported_rates: Option<Vec<f64>>,

    rsn: Option<RSNVersion>,

    channel: Option<u8>,
}

fn check<T: std::fmt::Debug + std::cmp::PartialEq>(original: Option<T>, test: Option<T>, s: &str) {
    assert_eq!(original.is_some(), test.is_some(), "{}.is_some()", s);
    if let Some(test) = test {
        assert_eq!(original.unwrap(), test, "{}", s);
    }
}

#[allow(clippy::cognitive_complexity)]
fn test_test_item(test_item: TestItem) {
    let frame = Frame::new(test_item.bytes);

    if let Some(version) = test_item.version {
        assert_eq!(frame.version(), version, "version");
    }

    if let Some(type_) = test_item.type_ {
        assert_eq!(frame.type_(), type_, "type_");
    }

    assert_eq!(frame.subtype(), test_item.subtype.unwrap(), "subtype");

    if let Some(ds_status) = test_item.ds_status {
        assert_eq!(frame.ds_status(), ds_status, "ds_status");
    }

    assert_eq!(
        frame.more_fragments(),
        test_item.more_fragments,
        "more_fragments"
    );
    assert_eq!(frame.retry(), test_item.retry, "retry");
    assert_eq!(frame.pwr_mgt(), test_item.pwr_mgt, "pwr_mgt");
    assert_eq!(frame.more_data(), test_item.more_data, "more_data");
    assert_eq!(frame.protected(), test_item.protected, "protected");
    assert_eq!(frame.order(), test_item.order, "order");

    if let Some(duration_id) = test_item.duration_id {
        assert_eq!(frame.duration_or_id(), duration_id, "duration_id");
    }

    assert_eq!(
        frame.receiver_address(),
        test_item.receiver_address,
        "receiver_address"
    );

    let transmitter_address;
    let destination_address;
    let source_address;
    let bssid_address;
    let station_address;
    let fragment_number;
    let sequence_number;

    if let Some(next_layer) = frame.next_layer() {
        match next_layer {
            FrameLayer::Management(layer) => {
                transmitter_address = layer.transmitter_address();
                destination_address = layer.destination_address();
                source_address = layer.source_address();
                bssid_address = layer.bssid_address();
                station_address = layer.station_address();

                fragment_number = Some(layer.fragment_number());
                sequence_number = Some(layer.sequence_number());

                let ssid = {
                    if let Some(layer) = layer.next_layer() {
                        match layer {
                            ManagementFrameLayer::Beacon(beacon_frame) => beacon_frame.ssid(),
                            ManagementFrameLayer::ProbeRequest(probe_request_frame) => {
                                probe_request_frame.ssid()
                            }
                            ManagementFrameLayer::ProbeResponse(probe_response_frame) => {
                                probe_response_frame.ssid()
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                };

                check(ssid, test_item.ssid, "ssid");

                if let Some(ref layer) = layer.next_layer() {
                    match layer {
                        ManagementFrameLayer::Beacon(ref beacon_frame) => {
                            assert_eq!(
                                beacon_frame.timestamp(),
                                test_item.timestamp.unwrap(),
                                "timestamp",
                            );

                            assert_eq!(
                                ((beacon_frame.beacon_interval() * 1_000_000f64).round()) as i64,
                                (test_item.beacon_interval.unwrap() * 1_000_000f64) as i64,
                                "beacon_interval",
                            );

                            assert_eq!(
                                beacon_frame.capabilities_info(),
                                test_item.capabilities_info.unwrap(),
                                "capabilities_info",
                            );

                            check(
                                beacon_frame.tagged_parameters().unwrap().supported_rates(),
                                test_item.supported_rates,
                                "supported_rates",
                            );

                            check(
                                beacon_frame.tagged_parameters().unwrap().rsn(),
                                test_item.rsn,
                                "rsn",
                            );

                            check(
                                beacon_frame.tagged_parameters().unwrap().channel(),
                                test_item.channel,
                                "channel",
                            );
                        }

                        ManagementFrameLayer::AssociationRequest(ref association_request_frame) => {
                            assert_eq!(
                                association_request_frame.capabilities_info(),
                                test_item.capabilities_info.unwrap(),
                                "capabilities_info",
                            );

                            check(
                                association_request_frame
                                    .tagged_parameters()
                                    .unwrap()
                                    .supported_rates(),
                                test_item.supported_rates,
                                "supported_rates",
                            );
                        }

                        ManagementFrameLayer::AssociationResponse(
                            ref association_response_frame,
                        ) => {
                            assert_eq!(
                                association_response_frame.capabilities_info(),
                                test_item.capabilities_info.unwrap(),
                                "capabilities_info",
                            );

                            check(
                                association_response_frame
                                    .tagged_parameters()
                                    .unwrap()
                                    .supported_rates(),
                                test_item.supported_rates,
                                "supported_rates",
                            );
                        }

                        _ => {}
                    }
                }
            }
            FrameLayer::Control(layer) => {
                transmitter_address = layer.transmitter_address();
                destination_address = layer.destination_address();
                source_address = layer.source_address();
                bssid_address = layer.bssid_address();
                station_address = layer.station_address();
                fragment_number = None;
                sequence_number = None;
            }
            FrameLayer::Data(layer) => {
                transmitter_address = layer.transmitter_address();
                destination_address = layer.destination_address();
                source_address = layer.source_address();
                bssid_address = layer.bssid_address();
                station_address = layer.station_address();
                fragment_number = Some(layer.fragment_number());
                sequence_number = Some(layer.sequence_number());
            }
        }

        check(
            transmitter_address,
            test_item.transmitter_address,
            "transmitter_address",
        );

        check(
            destination_address,
            test_item.destination_address,
            "destination_address",
        );

        check(source_address, test_item.source_address, "source_address");

        check(bssid_address, test_item.bssid_address, "bssid_address");

        check(
            station_address,
            test_item.station_address,
            "station_address",
        );

        check(
            fragment_number,
            test_item.fragment_number,
            "fragment_number",
        );

        check(
            sequence_number,
            test_item.sequence_number,
            "sequence_number",
        );
    } else {
        //
    }
}

// Management
include!("./packets/beacon.rs");
include!("./packets/authentication.rs");
include!("./packets/authentication_response.rs");
include!("./packets/deauthentication.rs");
include!("./packets/disassociate.rs");
include!("./packets/association_request.rs");
include!("./packets/association_response.rs");
include!("./packets/probe_request.rs");
include!("./packets/probe_response.rs");
include!("./packets/beacon_ciphers.rs");

// Control
include!("./packets/power_save_poll.rs");
include!("./packets/ack.rs");
include!("./packets/cts.rs");
include!("./packets/rts.rs");
include!("./packets/block_ack.rs");
include!("./packets/cf_end.rs");
include!("./packets/block_ack_request.rs");

// Data
include!("./packets/data.rs");
include!("./packets/data_layer.rs");
include!("./packets/qos_data.rs");
include!("./packets/null_data.rs");

include!("./packets/very_bad.rs");

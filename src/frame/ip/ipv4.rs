use super::IPVersion;

pub struct IPv4Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> IPv4Frame<'a> {
  pub fn new(bytes: &'a [u8]) -> IPv4Frame<'a> {
    IPv4Frame { bytes }
  }

  pub fn version(&self) -> IPVersion {
    match self.bytes[0] >> 4 {
      4 => IPVersion::IPv4,
      6 => IPVersion::IPv6,
      _ => panic!("other IPv4 version"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::ip::*;

  const IPV4_TCP_ACK_PACKET: [u8; 60] = [
    0x45, 0x10, 0x00, 0x3c, 0x46, 0x3c, 0x40, 0x00, 0x40, 0x06, 0x73, 0x1c, 0xc0, 0xa8, 0x00, 0x02,
    0xc0, 0xa8, 0x00, 0x01, 0x06, 0x0e, 0x00, 0x17, 0x99, 0xc5, 0xa0, 0xec, 0x00, 0x00, 0x00, 0x00,
    0xa0, 0x02, 0x7d, 0x78, 0xe0, 0xa3, 0x00, 0x00, 0x02, 0x04, 0x05, 0xb4, 0x04, 0x02, 0x08, 0x0a,
    0x00, 0x9c, 0x27, 0x24, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x03, 0x00,
  ];

  #[test]
  fn test_ipv4() {
    let ipv4_frame = IPv4Frame::new(&IPV4_TCP_ACK_PACKET[..]);
    assert_eq!(ipv4_frame.version(), IPVersion::IPv4);
  }
}

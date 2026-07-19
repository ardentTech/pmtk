use heapless::String;
use crate::packet::PmtkPacket;

const PKT_TYPE: u16 = 101;

// $PMTK101*32\r\n
pub struct HotStart {}

impl<'a> Into<PmtkPacket> for HotStart {
    fn into(self) -> PmtkPacket {
        PmtkPacket::new(String::default(), PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        // TODO should check into and not encoding
        let packet: PmtkPacket = HotStart {}.into();
        assert_eq!("$PMTK101*32\r\n", packet.encode());
    }
}
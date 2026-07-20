use heapless::String;
use crate::packet::PmtkPacket;

// $PMTK104*37\r\n
const PKT_TYPE: u16 = 104;

pub struct FullColdStart {}

impl Into<PmtkPacket> for FullColdStart {
    fn into(self) -> PmtkPacket {
        PmtkPacket::new(String::default(), PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        let packet: PmtkPacket = FullColdStart {}.into();
        assert_eq!("$PMTK104*37\r\n", packet.encode());
    }
}
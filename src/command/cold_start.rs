use heapless::String;
use crate::packet::PmtkPacket;

// $PMTK103*30\r\n
const PKT_TYPE: u16 = 103;

pub struct ColdStart {}

impl Into<PmtkPacket> for ColdStart {
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
        let packet: PmtkPacket = ColdStart {}.into();
        assert_eq!("$PMTK103*30\r\n", packet.encode());
    }
}
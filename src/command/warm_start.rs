use heapless::String;
use crate::packet::PmtkPacket;

const PKT_TYPE: u16 = 102;

// $PMTK102*31
pub struct WarmStart {}

impl<'a> Into<PmtkPacket> for WarmStart {
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
        let packet: PmtkPacket = WarmStart {}.into();
        assert_eq!("$PMTK102*31\r\n", packet.encode());
    }
}
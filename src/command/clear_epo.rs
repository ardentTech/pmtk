use heapless::String;
use crate::packet::PmtkPacket;

// $PMTK127*36\r\n
const PKT_TYPE: u16 = 127;

pub struct ClearEpo {}

impl Into<PmtkPacket> for ClearEpo {
    fn into(self) -> PmtkPacket {
        PmtkPacket::new(String::default(), PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        let packet: PmtkPacket = ClearEpo {}.into();
        assert_eq!("$PMTK127*36\r\n", packet.encode());
    }
}
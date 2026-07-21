use crate::command::util::encode_data_field;
use crate::packet::PmtkPacket;

// $PMTK352,1*2B\r\n
const PKT_TYPE: u16 = 352;

pub struct SetStopQzss(bool);

impl Into<PmtkPacket> for SetStopQzss {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.0 as u32]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_false_ok() {
        let packet: PmtkPacket = SetStopQzss(false).into();
        assert_eq!("$PMTK352,0*2A\r\n", packet.encode());
    }

    #[test]
    fn into_pmtk_packet_true_ok() {
        let packet: PmtkPacket = SetStopQzss(true).into();
        assert_eq!("$PMTK352,1*2B\r\n", packet.encode());
    }
}
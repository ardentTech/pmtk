use crate::command::util::encode_data_field;
use crate::packet::PmtkPacket;

// $PMTK351,1*28\r\n
const PKT_TYPE: u16 = 351;

pub struct SetSupportQzssNmea(bool);

impl Into<PmtkPacket> for SetSupportQzssNmea {
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
        let packet: PmtkPacket = SetSupportQzssNmea(false).into();
        assert_eq!("$PMTK351,0*29\r\n", packet.encode());
    }

    #[test]
    fn into_pmtk_packet_true_ok() {
        let packet: PmtkPacket = SetSupportQzssNmea(true).into();
        assert_eq!("$PMTK351,1*28\r\n", packet.encode());
    }
}
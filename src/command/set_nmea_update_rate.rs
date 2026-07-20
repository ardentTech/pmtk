use crate::command::util::encode_data_field;
use crate::packet::PmtkPacket;

// $PMTK220,1000*1F\r\n
const PKT_TYPE: u16 = 220;

pub struct SetNmeaUpdateRate {
    ms: u32 // TODO constrain 100..=10_000
}

impl Into<PmtkPacket> for SetNmeaUpdateRate {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.ms]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        // TODO should check into and not encoding
        let packet: PmtkPacket = SetNmeaUpdateRate { ms: 1000u32 }.into();
        assert_eq!("$PMTK220,1000*1F\r\n", packet.encode());
    }
}
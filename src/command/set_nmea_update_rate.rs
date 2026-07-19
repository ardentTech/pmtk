use core::fmt::Write;
use heapless::String;
use crate::packet::{DataField, PAYLOAD_LEN, PmtkPacket};

const PKT_TYPE: u16 = 220;

// $PMTK220,1000*1F\r\n
pub struct SetNmeaUpdateRate {
    ms: u32 // TODO constrain 100..=10_000
}

impl Into<PmtkPacket> for SetNmeaUpdateRate {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.ms]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}

fn encode_data_field<const N: usize>(data: [u32; N]) -> DataField {
    let mut data_field = String::new();
    for c in data {
        write!(data_field, ",{}", c).unwrap();
    }
    data_field
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

    #[test]
    fn encode_data_field_ok() {
        let data = [1000u32];
        assert_eq!(",1000", encode_data_field(data));
    }
}
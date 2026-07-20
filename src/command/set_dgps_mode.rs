use crate::command::util::encode_data_field;
use crate::packet::PmtkPacket;

// $PMTK301,1*2D\r\n
const PKT_TYPE: u16 = 301;

enum DgpsMode {
    None = 0x0,
    RTCM = 0x1,
    WAAS = 0x2,
}

pub struct SetDgpsMode {
    mode: DgpsMode
}

impl Into<PmtkPacket> for SetDgpsMode {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.mode as u32]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        let packet: PmtkPacket = SetDgpsMode { mode: DgpsMode::RTCM }.into();
        assert_eq!("$PMTK301,1*2D\r\n", packet.encode());
    }
}
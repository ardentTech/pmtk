use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::dgps_mode::DgpsModeDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDgpsModeCmd(pub DgpsModeDt);

impl PmtkSentence for SetDgpsModeCmd {
    const PKT_TYPE: u16 = 301;
}

impl PmtkBiDir for SetDgpsModeCmd {
    type Dt = AckDt;
}

impl PmtkCmd for SetDgpsModeCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::packet::DataField;
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = SetDgpsModeCmd(DgpsModeDt::RTCM);
        let packet = PmtkPacket {
            checksum: 0x2d,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetDgpsModeCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}
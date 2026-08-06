use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

pub struct AicModeCmd(pub bool);

impl PmtkSentence for AicModeCmd {
    const PKT_TYPE: u16 = 286;
}

impl PmtkBiDir for AicModeCmd {
    type Dt = AckDt;
}

impl PmtkCmd for AicModeCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::packet::{DataField, PmtkPacket};
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = AicModeCmd(true);
        let packet = PmtkPacket {
            checksum: 0x23,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: AicModeCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}
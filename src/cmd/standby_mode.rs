use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct StandbyModeCmd;

impl PmtkSentence for StandbyModeCmd {
    const PKT_TYPE: u16 = 161;
}

impl PmtkCmd for StandbyModeCmd {
    type DataType = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([0]);
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
        let cmd = StandbyModeCmd;
        let packet = PmtkPacket {
            checksum: 0x28,
            data_field: Some(DataField::from_str(",0").unwrap()),
            pkt_type: StandbyModeCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
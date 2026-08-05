use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CmdType {
    Query = 0x0,
    Set = 0x1,
    ResultForQueryOperation = 0x2
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct EasyEnableCmd { // TODO this might be better as a Response type?
    cmd_type: CmdType,
    enable: bool,
}

impl PmtkSentence for EasyEnableCmd {
    const PKT_TYPE: u16 = 869;
}

impl PmtkCmd for EasyEnableCmd {
    type DataType = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.cmd_type as u8, self.enable as u8]);
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
        let cmd = EasyEnableCmd { cmd_type: CmdType::Set, enable: true };
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: Some(DataField::from_str(",1,1").unwrap()),
            pkt_type: EasyEnableCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
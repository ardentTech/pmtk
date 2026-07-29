use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CmdType {
    Query = 0x0,
    Set = 0x1,
    ResultForQueryOperation = 0x2
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct EasyEnable {
    cmd_type: CmdType,
    enable: bool,
}

impl Message for EasyEnable {
    const PKT_TYPE: u16 = 869;
}

impl Command for EasyEnable {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.cmd_type as u8, self.enable as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::types::{DataField, PmtkPacket};
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = EasyEnable { cmd_type: CmdType::Set, enable: true };
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: Some(DataField::from_str(",1,1").unwrap()),
            pkt_type: EasyEnable::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
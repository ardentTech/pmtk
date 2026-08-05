use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

pub struct SetSbasEnabledCmd(bool);

impl Message for SetSbasEnabledCmd {
    const PKT_TYPE: u16 = 313;
}

impl Command for SetSbasEnabledCmd {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::types::DataField;
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = SetSbasEnabledCmd(true);
        let packet = PmtkPacket {
            checksum: 0x2e,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetSbasEnabledCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
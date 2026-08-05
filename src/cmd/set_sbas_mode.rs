use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSbasModeCmd(pub SbasModeDt);

impl Message for SetSbasModeCmd {
    const PKT_TYPE: u16 = 319;
}
impl Command for SetSbasModeCmd {
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
        let cmd = SetSbasModeCmd(SbasModeDt::Testing);
        let packet = PmtkPacket {
            checksum: 0x25,
            data_field: Some(DataField::from_str(",0").unwrap()),
            pkt_type: SetSbasModeCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
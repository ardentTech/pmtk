use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSbasMode {
    mode: SbasModeDt,
}

impl Message for SetSbasMode {
    const PKT_TYPE: u16 = 319;
}
impl Command for SetSbasMode {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.mode as u8]);
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
        let cmd = SetSbasMode { mode: SbasModeDt::Testing };
        let packet = PmtkPacket {
            checksum: 0x25,
            data_field: Some(DataField::from_str(",0").unwrap()),
            pkt_type: SetSbasMode::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
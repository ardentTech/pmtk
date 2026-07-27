use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

// TODO MT3339 only

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct StandbyMode;

impl Message for StandbyMode {
    const PKT_TYPE: u16 = 161;
}

impl Command for StandbyMode {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([0]);
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
        let cmd = StandbyMode;
        let packet = PmtkPacket {
            checksum: 0x28,
            data_field: Some(DataField::from_str(",0").unwrap()),
            pkt_type: StandbyMode::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
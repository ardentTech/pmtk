use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

pub struct AicMode(pub bool);

impl Message for AicMode {
    const PKT_TYPE: u16 = 286;
}

impl Command for AicMode {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
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
        let cmd = AicMode(true);
        let packet = PmtkPacket {
            checksum: 0x23,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: AicMode::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
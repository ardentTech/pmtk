use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

pub struct SetSupportQzssNmea(bool);

impl Message for SetSupportQzssNmea {
    const PKT_TYPE: u16 = 351;
}

impl Command for SetSupportQzssNmea {
    type Response = Ack;

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
        let cmd = SetSupportQzssNmea(true);
        let packet = PmtkPacket {
            checksum: 0x28,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetSupportQzssNmea::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

pub struct SetStopQzss(pub bool);

impl Message for SetStopQzss {
    const PKT_TYPE: u16 = 352;
}

impl Command for SetStopQzss {
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
        let cmd = SetStopQzss(true);
        let packet = PmtkPacket {
            checksum: 0x2B,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetStopQzss::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
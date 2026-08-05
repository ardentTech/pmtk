use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkSentence};
use crate::packet::PmtkPacket;

pub struct SetStopQzssCmd(pub bool);

impl PmtkSentence for SetStopQzssCmd {
    const PKT_TYPE: u16 = 352;
}

impl PmtkCmd for SetStopQzssCmd {
    type DataType = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
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
        let cmd = SetStopQzssCmd(true);
        let packet = PmtkPacket {
            checksum: 0x2B,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetStopQzssCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
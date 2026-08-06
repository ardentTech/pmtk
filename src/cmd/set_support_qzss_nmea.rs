use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

pub struct SetSupportQzssNmeaCmd(pub bool);

impl PmtkSentence for SetSupportQzssNmeaCmd {
    const PKT_TYPE: u16 = 351;
}

impl PmtkBiDir for SetSupportQzssNmeaCmd {
    type Dt = AckDt;
}

impl PmtkCmd for SetSupportQzssNmeaCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
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
        let cmd = SetSupportQzssNmeaCmd(true);
        let packet = PmtkPacket {
            checksum: 0x28,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetSupportQzssNmeaCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}
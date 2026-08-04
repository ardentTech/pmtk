use crate::cmd::util::encode_data_field;
use crate::dt::ack::AckDt;
use crate::error::PmtkError;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDatum(u8);

impl SetDatum {
    fn new(datum: u8) -> Result<Self, PmtkError> {
        if !(0..=222).contains(&datum) {
            return Err(PmtkError::InputOutOfRange)
        }
        Ok(Self { 0: datum })
    }
}

impl Message for SetDatum {
    const PKT_TYPE: u16 = 330;
}

impl Command for SetDatum {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0]);
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
        let cmd = SetDatum(2);
        let packet = PmtkPacket {
            checksum: 0x2c,
            data_field: Some(DataField::from_str(",2").unwrap()),
            pkt_type: SetDatum::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
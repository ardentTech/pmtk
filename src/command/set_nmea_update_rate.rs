use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

const MIN_MS: u16 = 100;
const MAX_MS: u16 = 10_000;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaUpdateRate {
    ms: u16
}
impl SetNmeaUpdateRate {
    pub fn new(ms: u16) -> Result<SetNmeaUpdateRate, PmtkError> {
        if ms < MIN_MS || ms > MAX_MS {
            Err(PmtkError::InvalidNmeaUpdateRate(ms))
        } else {
            Ok(SetNmeaUpdateRate { ms })
        }
    }
}

impl Message for SetNmeaUpdateRate {
    const PKT_TYPE: u16 = 220;
}

impl Command for SetNmeaUpdateRate {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.ms as u32]);
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
        let cmd = SetNmeaUpdateRate { ms: 1000 };
        let packet = PmtkPacket {
            checksum: 0x1f,
            data_field: Some(DataField::from_str(",1000").unwrap()),
            pkt_type: SetNmeaUpdateRate::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaUpdateRate::new(MAX_MS + 1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaUpdateRate::new(MIN_MS).is_ok());
    }
}
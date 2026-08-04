use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaBaudRate {
    bps: u32,
}

impl SetNmeaBaudRate {
    pub fn new(rate: u32) -> Result<Self, PmtkError> {
        if ![0, 4800, 9600, 14400, 19200, 38400, 57600, 115200].contains(&rate) {
            return Err(PmtkError::InvalidChoice(rate));
        }
        Ok(Self { bps: rate })
    }
}

impl Message for SetNmeaBaudRate {
    const PKT_TYPE: u16 = 251;
}

impl Command for SetNmeaBaudRate {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.bps]);
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
        let cmd = SetNmeaBaudRate { bps: 38400 };
        let packet = PmtkPacket {
            checksum: 0x27,
            data_field: Some(DataField::from_str(",38400").unwrap()),
            pkt_type: SetNmeaBaudRate::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaBaudRate::new(1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaBaudRate::new(38400).is_ok());
    }
}
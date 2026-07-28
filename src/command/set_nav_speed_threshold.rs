use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

pub struct SetNavSpeedThreshold {
    threshold: f32
}

impl SetNavSpeedThreshold {
    pub fn new(threshold: f32) -> Result<Self, PmtkError> {
        if ![0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.5, 2.0].contains(&threshold) {
            return Err(PmtkError::InvalidNavSpeedThreshold(threshold));
        }
        Ok(Self { threshold })
    }
}

#[cfg(not(feature = "mt3339"))]
impl Message for SetNavSpeedThreshold {
    const PKT_TYPE: u16 = 397;
}

#[cfg(feature = "mt3339")]
impl Message for SetNavSpeedThreshold {
    const PKT_TYPE: u16 = 386;
}

impl Command for SetNavSpeedThreshold {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.threshold]);
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
        let cmd = SetNavSpeedThreshold { threshold: 0.2 };
        let packet = PmtkPacket {
            checksum: 0x3f,
            data_field: Some(DataField::from_str(",0.2").unwrap()),
            pkt_type: SetNavSpeedThreshold::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNavSpeedThreshold::new(0.5).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNavSpeedThreshold::new(0.2).is_ok());
    }
}
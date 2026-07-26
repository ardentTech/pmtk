use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaUpdateRate {
    ms: u16 // TODO constrain 100..=10_000
}

impl Message for SetNmeaUpdateRate {
    const PKT_TYPE: u16 = 220;
}

impl Command for SetNmeaUpdateRate {
    type Response = Ack;

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
}
use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusConfig {
    pub log_frequency: u8
}

impl Message for LocusConfig {
    const PKT_TYPE: u16 = 187;
}

impl Command for LocusConfig {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([1, self.log_frequency]);
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
        let cmd = LocusConfig { log_frequency: 5 };
        let packet = PmtkPacket {
            checksum: 0x38,
            data_field: Some(DataField::from_str(",1,5").unwrap()),
            pkt_type: LocusConfig::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
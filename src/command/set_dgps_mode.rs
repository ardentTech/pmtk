use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::AckDt;
use crate::response::dgps_mode::DgpsModeDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDgpsMode { // TODO tuple struct
    mode: DgpsModeDt,
}

impl Message for SetDgpsMode {
    const PKT_TYPE: u16 = 301;
}
impl Command for SetDgpsMode {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.mode as u8]);
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
        let cmd = SetDgpsMode { mode: DgpsModeDt::RTCM };
        let packet = PmtkPacket {
            checksum: 0x2d,
            data_field: Some(DataField::from_str(",1").unwrap()),
            pkt_type: SetDgpsMode::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}
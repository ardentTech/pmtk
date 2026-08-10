use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::dgps_mode::DgpsModeDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDgpsModeCmd(pub DgpsModeDt);

impl Packet for SetDgpsModeCmd {
    const PKT_TYPE: u16 = 301;
}

impl Request for SetDgpsModeCmd {
    type R = AckDt;
}

impl Cmd for SetDgpsModeCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK301,1*2D\r\n", SetDgpsModeCmd(DgpsModeDt::RTCM).serialize().unwrap());
    }
}
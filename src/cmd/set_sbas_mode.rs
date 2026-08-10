use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSbasModeCmd(pub SbasModeDt);

impl Packet for SetSbasModeCmd {
    const PKT_TYPE: u16 = 319;
}

impl Request for SetSbasModeCmd {
    type R = AckDt;
}

impl Cmd for SetSbasModeCmd {
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
        assert_eq!("$PMTK319,0*25\r\n", SetSbasModeCmd(SbasModeDt::Testing).serialize().unwrap());
    }
}
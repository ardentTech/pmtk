use crate::dt::sbas_mode::SbasModeDt;
use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSbasModeCmd(SbasModeDt);

impl SetSbasModeCmd {
    pub fn new(mode: SbasModeDt) -> Self {
        Self(mode)
    }
}

impl Packet for SetSbasModeCmd {
    const PKT_TYPE: PktType = [51, 49, 57]; // 319
}

impl CmdQ for SetSbasModeCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
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
use crate::dt::dgps_mode::DgpsModeDt;
use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDgpsModeCmd(DgpsModeDt);

impl SetDgpsModeCmd {
    pub fn new(mode: DgpsModeDt) -> Self {
        Self(mode)
    }
}

impl Packet for SetDgpsModeCmd {
    const PKT_TYPE: PktType = [51, 48, 49]; // 301
}

impl CmdQ for SetDgpsModeCmd {
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
        assert_eq!("$PMTK301,1*2D\r\n", SetDgpsModeCmd(DgpsModeDt::RTCM).serialize().unwrap());
    }
}
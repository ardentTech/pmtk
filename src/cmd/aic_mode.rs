use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct AicModeCmd(bool);

impl AicModeCmd {
    pub fn new(enable: bool) -> Self {
        Self(enable)
    }
}

impl Packet for AicModeCmd {
    const PKT_TYPE: u16 = 286;
}

impl CmdQ for AicModeCmd {
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
        assert_eq!("$PMTK286,1*23\r\n", AicModeCmd(true).serialize().unwrap());
    }
}
use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct EasyEnableCmd(bool);

impl EasyEnableCmd {
    pub fn new(enable: bool) -> Self {
        Self(enable)
    }
}

impl Packet for EasyEnableCmd {
    const PKT_TYPE: PktType = [56, 54, 57]; // 869
}

impl CmdQ for EasyEnableCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([1, self.0 as u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK869,1,1*35\r\n", EasyEnableCmd(true).serialize().unwrap());
    }
}
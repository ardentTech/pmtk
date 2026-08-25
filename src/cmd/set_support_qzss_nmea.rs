use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSupportQzssNmeaCmd(bool);

impl SetSupportQzssNmeaCmd {
    pub fn new(enable: bool) -> Self {
        Self(enable)
    }
}

impl Packet for SetSupportQzssNmeaCmd {
    const PKT_TYPE: u16 = 351;
}

impl CmdQ for SetSupportQzssNmeaCmd {
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
        assert_eq!("$PMTK351,1*28\r\n", SetSupportQzssNmeaCmd(true).serialize().unwrap());
    }
}
use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{Cmd, Packet, Request};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetStopQzssCmd(bool);

impl SetStopQzssCmd {
    pub fn new(enable: bool) -> Self {
        Self(!enable)
    }
}

impl Packet for SetStopQzssCmd {
    const PKT_TYPE: u16 = 352;
}

impl Request for SetStopQzssCmd {}

impl Cmd for SetStopQzssCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_enable_ok() {
        assert_eq!("$PMTK352,0*2A\r\n", SetStopQzssCmd::new(true).serialize().unwrap());
    }

    #[test]
    fn serialize_disable_ok() {
        assert_eq!("$PMTK352,1*2B\r\n", SetStopQzssCmd::new(false).serialize().unwrap());
    }
}
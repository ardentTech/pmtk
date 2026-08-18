use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::{PmtkPacket, SerializedPacket};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetDatumCmd(u8);

impl SetDatumCmd {
    pub fn new(datum: u8) -> Result<Self, PmtkError> {
        if !(0..=222).contains(&datum) {
            return Err(PmtkError::OutOfRange(0, 222, datum as u32));
        }
        Ok(Self { 0: datum })
    }
}

impl Packet for SetDatumCmd {
    const PKT_TYPE: u16 = 330;
}

impl Request for SetDatumCmd {}

impl Cmd for SetDatumCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK330,2*2C\r\n", SetDatumCmd(2).serialize().unwrap());
    }
}
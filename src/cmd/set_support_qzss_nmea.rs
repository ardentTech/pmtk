use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

pub struct SetSupportQzssNmeaCmd(pub bool);

impl Packet for SetSupportQzssNmeaCmd {
    const PKT_TYPE: u16 = 351;
}

impl Request for SetSupportQzssNmeaCmd {}

impl Cmd for SetSupportQzssNmeaCmd {
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
        assert_eq!("$PMTK351,1*28\r\n", SetSupportQzssNmeaCmd(true).serialize().unwrap());
    }
}
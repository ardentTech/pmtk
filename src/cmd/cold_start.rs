use heapless::String;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ColdStartCmd;

impl Packet for ColdStartCmd {
    const PKT_TYPE: u16 = 103;
}

impl Request for ColdStartCmd {
    type R = AckDt;
}

impl Cmd for ColdStartCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK103*30\r\n", ColdStartCmd {}.serialize().unwrap());
    }
}
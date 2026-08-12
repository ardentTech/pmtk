use heapless::String;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct FullColdStartCmd;

impl Packet for FullColdStartCmd {
    const PKT_TYPE: u16 = 104;
}

impl Request for FullColdStartCmd {}

impl Cmd for FullColdStartCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK104*37\r\n", FullColdStartCmd {}.serialize().unwrap());
    }
}
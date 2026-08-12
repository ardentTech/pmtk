use heapless::String;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct WarmStartCmd;

impl Packet for WarmStartCmd {
    const PKT_TYPE: u16 = 102;
}

impl Request for WarmStartCmd {}

impl Cmd for WarmStartCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK102*31\r\n", WarmStartCmd {}.serialize().unwrap());
    }
}
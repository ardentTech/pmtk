use heapless::String;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct HotStartCmd;

impl Packet for HotStartCmd {
    const PKT_TYPE: u16 = 101;
}

impl Request for HotStartCmd {
    type R = AckDt;
}

impl Cmd for HotStartCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK101*32\r\n", HotStartCmd {}.serialize().unwrap());
    }
}
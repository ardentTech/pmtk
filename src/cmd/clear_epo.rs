use heapless::String;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ClearEpoCmd;

impl Packet for ClearEpoCmd {
    const PKT_TYPE: u16 = 127;
}

impl Request for ClearEpoCmd {
    type R = AckDt;
}

impl Cmd for ClearEpoCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        let cmd = ClearEpoCmd;
        assert_eq!("$PMTK127*36\r\n", cmd.serialize().unwrap());
    }
}
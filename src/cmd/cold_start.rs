use crate::packet::PktType;
use crate::traits::{CmdQ, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ColdStartCmd;

impl Packet for ColdStartCmd {
    const PKT_TYPE: PktType = [49, 48, 51]; // 103
}

impl CmdQ for ColdStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK103*30\r\n", ColdStartCmd {}.serialize().unwrap());
    }
}
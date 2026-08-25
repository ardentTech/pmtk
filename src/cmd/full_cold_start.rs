use crate::traits::{CmdQ, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct FullColdStartCmd;

impl Packet for FullColdStartCmd {
    const PKT_TYPE: u16 = 104;
}

impl CmdQ for FullColdStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK104*37\r\n", FullColdStartCmd {}.serialize().unwrap());
    }
}
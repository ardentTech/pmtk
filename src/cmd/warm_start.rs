use crate::packet::PktType;
use crate::traits::{CmdQ, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct WarmStartCmd;

impl Packet for WarmStartCmd {
    const PKT_TYPE: PktType = [49, 48, 50]; // 102
}

impl CmdQ for WarmStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK102*31\r\n", WarmStartCmd {}.serialize().unwrap());
    }
}
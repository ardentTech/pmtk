use crate::traits::{Cmd, Request, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct WarmStartCmd;

impl Packet for WarmStartCmd {
    const PKT_TYPE: u16 = 102;
}

impl Request for WarmStartCmd {}

impl Cmd for WarmStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK102*31\r\n", WarmStartCmd {}.serialize().unwrap());
    }
}
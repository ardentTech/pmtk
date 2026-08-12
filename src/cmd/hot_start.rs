use crate::traits::{Cmd, Request, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct HotStartCmd;

impl Packet for HotStartCmd {
    const PKT_TYPE: u16 = 101;
}

impl Request for HotStartCmd {}

impl Cmd for HotStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK101*32\r\n", HotStartCmd {}.serialize().unwrap());
    }
}
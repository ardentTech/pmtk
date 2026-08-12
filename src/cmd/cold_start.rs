use crate::traits::{Cmd, Request, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ColdStartCmd;

impl Packet for ColdStartCmd {
    const PKT_TYPE: u16 = 103;
}

impl Request for ColdStartCmd {}

impl Cmd for ColdStartCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK103*30\r\n", ColdStartCmd {}.serialize().unwrap());
    }
}
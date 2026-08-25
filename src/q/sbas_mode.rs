use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasModeQ {}

impl Packet for SbasModeQ {
    const PKT_TYPE: u16 = 419;
}

impl CmdQ for SbasModeQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK419*3E\r\n", SbasModeQ {}.serialize().unwrap());
    }
}
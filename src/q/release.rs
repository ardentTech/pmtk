use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct ReleaseQ {}

impl Packet for ReleaseQ {
    const PKT_TYPE: u16 = 605;
}

impl CmdQ for ReleaseQ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK605*31\r\n", ReleaseQ {}.serialize().unwrap());
    }
}
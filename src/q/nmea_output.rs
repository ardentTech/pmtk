use crate::packet::PktType;
use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NmeaOutputQ {}

impl Packet for NmeaOutputQ {
    const PKT_TYPE: PktType = [52, 49, 52]; // 414
}

impl CmdQ for NmeaOutputQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK414*33\r\n", NmeaOutputQ {}.serialize().unwrap());
    }
}
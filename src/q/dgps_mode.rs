use crate::packet::PktType;
use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DgpsModeQ {}
impl Packet for DgpsModeQ {
    const PKT_TYPE: PktType = [52, 48, 49]; // 401
}

impl CmdQ for DgpsModeQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK401*37\r\n", DgpsModeQ {}.serialize().unwrap());
    }
}
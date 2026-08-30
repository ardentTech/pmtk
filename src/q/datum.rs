use crate::packet::PktType;
use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DatumQ {}
impl Packet for DatumQ {
    const PKT_TYPE: PktType = [52, 51, 48]; // 430
}

impl CmdQ for DatumQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK430*35\r\n", DatumQ {}.serialize().unwrap());
    }
}
use crate::dt::datum::DatumDt;
use crate::traits::{Packet, Q, Request};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DatumQ {}
impl Packet for DatumQ {
    const PKT_TYPE: u16 = 430;
}

impl Request for DatumQ {
    type R = DatumDt;
}

impl Q for DatumQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK430*35\r\n", DatumQ {}.serialize().unwrap());
    }
}
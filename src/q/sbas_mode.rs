use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{Packet, Q, Request};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasModeQ {}
impl Packet for SbasModeQ {
    const PKT_TYPE: u16 = 419;
}

impl Request for SbasModeQ {
    type R = SbasModeDt;
}

impl Q for SbasModeQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK419*3E\r\n", SbasModeQ {}.serialize().unwrap());
    }
}
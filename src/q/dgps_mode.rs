use crate::traits::{Packet, Q, Request};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DgpsModeQ {}
impl Packet for DgpsModeQ {
    const PKT_TYPE: u16 = 401;
}

impl Request for DgpsModeQ {}

impl Q for DgpsModeQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK401*37\r\n", DgpsModeQ {}.serialize().unwrap());
    }
}
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number;
use crate::traits::{Dt, Packet};
use crate::packet::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReleaseDt {
    pub build_id: u16,
    pub s: Option<[u8; 16]>,
}

impl Packet for ReleaseDt {
    const PKT_TYPE: u16 = 705;
}

impl TryFrom<DataField> for ReleaseDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, s) = take_until(",").parse(i)?;
        let (i, _) = char(',').parse(i)?;
        let (_, build_id) = number::<u16>(i)?;

        let s = if s.is_empty() {
            None
        } else {
            let mut buf = [0u8; 16];
            s.as_bytes().iter().enumerate().for_each(|(i, b)| buf[i] = *b);
            Some(buf)
        };

        Ok(ReleaseDt { build_id, s })
    }
}

impl Dt for ReleaseDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",AXN_1.3,2102,ABCD").unwrap();
        let release = ReleaseDt::try_from(data_field).unwrap();
        assert_eq!(release.build_id, 2102);
        assert_eq!(release.s.unwrap(), [65, 88, 78, 95, 49, 46, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
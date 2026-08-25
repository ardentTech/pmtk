use nom::character::complete::char;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Dt, Packet};
use crate::packet::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DatumDt(pub u8);

impl Packet for DatumDt {
    const PKT_TYPE: u16 = 530;
}

impl TryFrom<DataField> for DatumDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, threshold) = number_in_range(i, 0, 222)?;
        Ok(Self(threshold))
    }
}

impl Dt for DatumDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",2").unwrap();
        let nav_threshold = DatumDt::try_from(data_field).unwrap();
        assert_eq!(nav_threshold.0, 2);
    }
}
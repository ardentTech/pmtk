use nom::character::complete::char;
use nom::number::complete::float;
use nom::Parser;
use crate::error::PmtkError;
use crate::traits::{Message, Response};
use crate::types::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThresholdDt(pub f32);

impl Message for NavThresholdDt {
    const PKT_TYPE: u16 = 527;
}

impl TryFrom<DataField> for NavThresholdDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, threshold) = float(i)?;
        Ok(Self(threshold))
    }
}

impl Response for NavThresholdDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",0.8").unwrap();
        let nav_threshold = NavThresholdDt::try_from(data_field).unwrap();
        assert_eq!(nav_threshold.0, 0.8);
    }
}
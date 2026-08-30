use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Dt, Packet};
use crate::packet::{DataField, PktType};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct EasyEnableDt(pub bool);

impl Packet for EasyEnableDt {
    const PKT_TYPE: PktType = [56, 54, 57]; // 869
}

impl TryFrom<DataField> for EasyEnableDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, _) = char('2').parse(i)?;
        let (i, _) = char(',').parse(i)?;
        let (_, enabled) = opt(|i| number_in_range::<u8>(i, 0, 1)).parse(i)?;

        if let Some(enabled) = enabled {
            Ok(EasyEnableDt(enabled == 1))
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Dt for EasyEnableDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",2,0").unwrap();
        let easy_enable = EasyEnableDt::try_from(data_field).unwrap();
        assert!(!easy_enable.0);
    }
}
use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Dt, Packet};
use crate::packet::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SbasEnabledDt(pub bool);

impl Packet for SbasEnabledDt {
    const PKT_TYPE: u16 = 513;
}

impl TryFrom<DataField> for SbasEnabledDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, enabled) = opt(|i| number_in_range::<u8>(i, 0, 1)).parse(i)?;
        if let Some(enabled) = enabled {
            Ok(SbasEnabledDt(enabled == 1))
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Dt for SbasEnabledDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::packet::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",1").unwrap();
        let sbas_enabled = SbasEnabledDt::try_from(data_field).unwrap();
        assert!(sbas_enabled.0);
    }
}
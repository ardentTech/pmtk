use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Message, Response};
use crate::types::DataField;

pub struct SbasEnabled(pub bool);

impl Message for SbasEnabled {
    const PKT_TYPE: u16 = 513;
}

impl TryFrom<DataField> for SbasEnabled {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, enabled) = opt(|i| number_in_range::<u8>(i, 0, 1)).parse(i)?;
        if let Some(enabled) = enabled {
            Ok(SbasEnabled(enabled == 1))
        } else {
            Err(PmtkError::Parsing) // TODO too generic?
        }
    }
}

impl Response for SbasEnabled {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::types::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",1").unwrap();
        let sbas_enabled = SbasEnabled::try_from(data_field).unwrap();
        assert!(sbas_enabled.0);
    }
}
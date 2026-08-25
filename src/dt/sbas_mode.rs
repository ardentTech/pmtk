use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Dt, Packet};
use crate::packet::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SbasModeDt {
    #[default]
    Testing = 0x0,
    Integrity = 0x1,
}

impl Packet for SbasModeDt {
    const PKT_TYPE: u16 = 519;
}

impl TryFrom<u8> for SbasModeDt {
    type Error = PmtkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SbasModeDt::Testing),
            0x1 => Ok(SbasModeDt::Integrity),
            _ => Err(PmtkError::OutOfRange(0x0, 0x1, value as u32))
        }
    }
}

impl TryFrom<DataField> for SbasModeDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, mode) = opt(|i| number_in_range::<u8>(i, 0, 3)).parse(i)?;

        if let Some(mode) = mode {
            SbasModeDt::try_from(mode).map_err(|_| PmtkError::Parsing)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Dt for SbasModeDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",1").unwrap();
        let dgps_mode = SbasModeDt::try_from(data_field).unwrap();
        assert_eq!(dgps_mode, SbasModeDt::Integrity);
    }
}
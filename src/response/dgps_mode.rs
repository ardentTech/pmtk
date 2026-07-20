use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::packet::{parse_number_in_range, DataField};
use crate::types::DgpsMode;

// $PMTK401,1*37\r\n
const PKT_TYPE: u16 = 401;

impl TryFrom<DataField> for DgpsMode {
    type Error = PmtkError;

    fn try_from(data_field: DataField) -> Result<Self, Self::Error> {
        let i = data_field.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, mode) = opt(|i| parse_number_in_range::<u8>(i, 0, 3)).parse(i)?;

        if let Some(mode) = mode {
            DgpsMode::try_from(mode).map_err(|_| PmtkError::Parsing)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn dgps_mode_try_from_ok() {
        let data_field = DataField::from_str(",1").unwrap();
        let mode = DgpsMode::try_from(data_field).unwrap();
        assert_eq!(mode, DgpsMode::RTCM);
    }
}
use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Dt, Packet};
use crate::packet::{DataField, PktType};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DgpsModeDt {
    #[default]
    None = 0x0,
    RTCM = 0x1,
    WAAS = 0x2,
}

impl Packet for DgpsModeDt {
    const PKT_TYPE: PktType = [53, 48, 49]; // 501
}

impl TryFrom<u8> for DgpsModeDt {
    type Error = PmtkError;
    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(DgpsModeDt::None),
            1 => Ok(DgpsModeDt::RTCM),
            2 => Ok(DgpsModeDt::WAAS),
            _ => Err(PmtkError::OutOfRange(0x0, 0x2, mode as u32)),
        }
    }
}

impl TryFrom<DataField> for DgpsModeDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, mode) = opt(|i| number_in_range::<u8>(i, 0, 3)).parse(i)?;

        if let Some(mode) = mode {
            DgpsModeDt::try_from(mode).map_err(|_| PmtkError::Parsing)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Dt for DgpsModeDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",1").unwrap();
        let dgps_mode = DgpsModeDt::try_from(data_field).unwrap();
        assert_eq!(dgps_mode, DgpsModeDt::RTCM);
    }
}
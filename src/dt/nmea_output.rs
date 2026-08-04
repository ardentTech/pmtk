use nom::character::complete::char;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number;
use crate::dt::nmea_output::Frequency::*;
use crate::traits::{Message, Response};
use crate::types::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Frequency {
    #[default]
    Disabled = 0x0,
    OnceEveryOnePositionFix = 0x1,
    OnceEveryTwoPositionFixes = 0x2,
    OnceEveryThreePositionFixes = 0x3,
    OnceEveryFourPositionFixes = 0x4,
    OnceEveryFivePositionFixes = 0x5,
}

impl TryFrom<u8> for Frequency {
    type Error = PmtkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Disabled),
            0x1 => Ok(OnceEveryOnePositionFix),
            0x2 => Ok(OnceEveryTwoPositionFixes),
            0x3 => Ok(OnceEveryThreePositionFixes),
            0x4 => Ok(OnceEveryFourPositionFixes),
            0x5 => Ok(OnceEveryFivePositionFixes),
            _ => Err(PmtkError::OutOfRange(0x0, 0x5, value as u32)),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NmeaOutputDt {
    pub gll: Frequency,
    pub rmc: Frequency,
    pub vtg: Frequency,
    pub gga: Frequency,
    pub gsa: Frequency,
    pub gsv: Frequency,
    pub mchn: Frequency,
}

impl Message for NmeaOutputDt {
    const PKT_TYPE: u16 = 514;
}

impl TryFrom<DataField> for NmeaOutputDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, gll) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, rmc) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, vtg) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, gga) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, gsa) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, gsv) = number::<u8>(i)?;
        // reserved
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, _) = number::<u8>(i)?;
        // end reserved
        let (i, _) = char(',').parse(i)?;
        let (_, mchn) = number::<u8>(i)?;

        Ok(Self {
            gll: Frequency::try_from(gll)?,
            rmc: Frequency::try_from(rmc)?,
            vtg: Frequency::try_from(vtg)?,
            gga: Frequency::try_from(gga)?,
            gsa: Frequency::try_from(gsa)?,
            gsv: Frequency::try_from(gsv)?,
            mchn: Frequency::try_from(mchn)?,
        })
    }
}

impl Response for NmeaOutputDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",1,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0,3").unwrap();
        let nmea_output = NmeaOutputDt::try_from(data_field).unwrap();
        assert_eq!(nmea_output.gll, OnceEveryOnePositionFix);
        assert_eq!(nmea_output.rmc, OnceEveryOnePositionFix);
        assert_eq!(nmea_output.vtg, OnceEveryOnePositionFix);
        assert_eq!(nmea_output.gga, OnceEveryOnePositionFix);
        assert_eq!(nmea_output.gsa, OnceEveryOnePositionFix);
        assert_eq!(nmea_output.gsv, OnceEveryFivePositionFixes);
        assert_eq!(nmea_output.mchn, OnceEveryThreePositionFixes);
    }
}
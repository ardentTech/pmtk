use nom::character::complete::{anychar, char};
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::packet::{DataField, PktType};
use crate::parse::{number, number_in_range};
use crate::traits::{Dt, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LogDt {
    pub serial: u16,
    pub full_stop: bool,
    pub mode: char,
    pub content: u8,
    pub interval: u8,
    pub distance: u8,
    pub speed: u8,
    pub status: u8,
    pub number: u16,
    pub percent: u8,
}

impl Packet for LogDt {
    const PKT_TYPE: PktType = [76, 79, 71]; // LOG
}

impl TryFrom<DataField> for LogDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, serial) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, full_stop) = opt(|i| number_in_range::<u8>(i, 0, 1)).parse(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, mode) = anychar(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, content) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, interval) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, distance) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, speed) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, status) = number::<u8>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, num) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (_, percent) = number::<u8>(i)?;

        if let Some(full_stop) = full_stop {
            Ok(LogDt {
                serial,
                full_stop: full_stop == 1,
                mode,
                content,
                interval,
                distance,
                speed,
                status,
                number: num,
                percent,
            })
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Dt for LogDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",456,0,b,31,2,0,0,0,3769,46").unwrap();
        let log = LogDt::try_from(data_field).unwrap();
        assert_eq!(log.serial, 456);
        assert_eq!(log.full_stop, false);
        assert_eq!(log.mode, 'b');
        assert_eq!(log.content, 31);
        assert_eq!(log.interval, 2);
        assert_eq!(log.distance, 0);
        assert_eq!(log.speed, 0);
        assert_eq!(log.status, 0);
        assert_eq!(log.number, 3769);
        assert_eq!(log.percent, 46);
    }
}
use nom::Parser;
use nom::character::complete::char;
use nom::combinator::opt;
use crate::error::PmtkError;
use crate::parse::number_in_range;
use crate::traits::{Packet, Response};
use crate::packet::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AckFlag {
    Invalid = 0x0,
    Unsupported = 0x1,
    ActionFailed = 0x2,
    ActionSucceeded = 0x3,
}

impl TryFrom<u8> for AckFlag {
    type Error = PmtkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(AckFlag::Invalid),
            0x1 => Ok(AckFlag::Unsupported),
            0x2 => Ok(AckFlag::ActionFailed),
            0x3 => Ok(AckFlag::ActionSucceeded),
            _ => Err(PmtkError::OutOfRange(0x0, 0x3, value as u32)),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AckDt {
    pub cmd: u16,
    pub flag: AckFlag
}

impl Packet for AckDt {
    const PKT_TYPE: u16 = 1;
}

impl TryFrom<DataField> for AckDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let mut comma = char(',');
        let (i, _) = comma(i)?;
        let (i, cmd) = opt(|i| number_in_range::<u16>(i, 0, 1000)).parse(i)?;
        let (i, _) = comma(i)?;
        let (_, flag) = opt(|i| number_in_range::<u8>(i, 0, 3)).parse(i)?;

        let mut res = Err(PmtkError::Parsing);
        if let Some(f) = flag {
            if let Some(cmd) = cmd {
                res = Ok(AckDt { cmd, flag: AckFlag::try_from(f)? })
            }
        }
        res
    }
}

impl Response for AckDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::packet::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",604,3").unwrap();
        let ack_data = AckDt::try_from(data_field).unwrap();
        assert_eq!(ack_data.cmd, 604);
        assert_eq!(ack_data.flag, AckFlag::ActionSucceeded);
    }
}
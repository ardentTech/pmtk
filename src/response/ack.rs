use nom::Parser;
use nom::character::complete::char;
use nom::combinator::opt;
use crate::error::PmtkError;
use crate::parser::parse_number_in_range;
use crate::traits::{Message, Response};
use crate::types::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
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
            _ => Err(PmtkError::InvalidAckFlag(value)),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Ack {
    cmd: u16,
    flag: AckFlag
}

impl Message for Ack {
    const PKT_TYPE: u16 = 1;
}

impl TryFrom<DataField> for Ack {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let mut comma = char(',');

        let (i, _) = comma(i)?;
        let (i, cmd) = opt(|i| parse_number_in_range::<u16>(i, 0, 1000)).parse(i)?;
        let (i, _) = comma(i)?;
        let (_, flag) = opt(|i| parse_number_in_range::<u8>(i, 0, 3)).parse(i)?;
        let flag = AckFlag::try_from(flag.unwrap())?;

        if let Some(cmd) = cmd {
            Ok(Ack { cmd, flag })
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Response for Ack {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use heapless::String;
    use super::*;
    use crate::types::DataField;

    struct AckData(String<242>);

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",604,3").unwrap();
        let ack_data = Ack::try_from(data_field).unwrap();
        assert_eq!(ack_data.cmd, 604);
        assert_eq!(ack_data.flag, AckFlag::ActionSucceeded);
    }
}
use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::parser::parse_number_in_range;
use crate::traits::{Message, Response};
use crate::types::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub enum SysMsg {
    Unknown = 0x0,
    Startup = 0x1,
    HostAidingEpoNotification = 0x2,
    NormalModeTransitionNotification = 0x3,
}

impl TryFrom<u8> for SysMsg {
    type Error = PmtkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SysMsg::Unknown),
            0x1 => Ok(SysMsg::Startup),
            0x2 => Ok(SysMsg::HostAidingEpoNotification),
            0x3 => Ok(SysMsg::NormalModeTransitionNotification),
            _ => Err(PmtkError::InvalidSysMsg(value)),
        }
    }
}

impl Message for SysMsg {
    const PKT_TYPE: u16 = 10;
}

impl TryFrom<DataField> for SysMsg {
    type Error = PmtkError;
    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, msg) = opt(|i| parse_number_in_range::<u8>(i, 0, 3)).parse(i)?;

        if let Some(msg) = msg {
            SysMsg::try_from(msg)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

impl Response for SysMsg {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::types::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",001").unwrap();
        let sys_msg = SysMsg::try_from(data_field).unwrap();
        assert_eq!(sys_msg, SysMsg::Startup);

    }
}
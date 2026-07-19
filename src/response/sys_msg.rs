use nom::character::complete::char;
use nom::combinator::opt;
use nom::Parser;
use crate::error::PmtkError;
use crate::packet::{parse_number_in_range, DataField};

// $PMTK010,001*2E\r\n
#[derive(Debug, PartialEq)]
pub enum SysMsg {
    Unknown = 0x0,
    Startup = 0x1,
    HostAidingEpo = 0x2,
    NormalModeTransitionOk = 0x3,
}
impl TryFrom<u8> for SysMsg {
    type Error = PmtkError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SysMsg::Unknown),
            0x1 => Ok(SysMsg::Startup),
            0x2 => Ok(SysMsg::HostAidingEpo),
            0x3 => Ok(SysMsg::NormalModeTransitionOk),
            _ => Err(PmtkError::Parsing)
        }
    }
}

impl TryFrom<DataField> for SysMsg {
    type Error = PmtkError;

    fn try_from(data_field: DataField) -> Result<Self, Self::Error> {
        let i = data_field.as_str();
        let (i, _) = char(',').parse(i)?;
        let (_, msg) = opt(|i| parse_number_in_range::<u8>(i, 0, 3)).parse(i)?;

        if let Some(msg) = msg {
            SysMsg::try_from(msg).map_err(|_| PmtkError::Parsing)
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
    fn sys_msg_try_from_data_field_ok() {
        let data_field = DataField::from_str(",001").unwrap();
        let sys_msg = SysMsg::try_from(data_field).unwrap();
        assert_eq!(sys_msg, SysMsg::Startup);
    }
}
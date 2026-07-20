// $PMTK011,MTKGPS*08\r\n

use core::str::FromStr;
use heapless::String;
use nom::character::complete::char;
use crate::error::PmtkError;
use crate::packet::{DATA_FIELD_LEN, DataField};

pub struct TxtMsg {
    msg: String<DATA_FIELD_LEN>
}

impl TryFrom<DataField> for TxtMsg {
    type Error = PmtkError;

    fn try_from(data_field: DataField) -> Result<Self, Self::Error> {
        let i = data_field.as_str();
        let (msg, _) = char(',')(i)?;
        // TODO this is weird bc msg will be DATA_FIELD_LEN - 1 when parsing succeeds
        Ok(TxtMsg { msg: String::from_str(msg).unwrap() })
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn txt_msg_try_from_data_field_ok() {
        let data_field = DataField::from_str(",MTKGPS").unwrap();
        let txt_msg = TxtMsg::try_from(data_field).unwrap();
        assert_eq!(txt_msg.msg, "MTKGPS");
    }
}
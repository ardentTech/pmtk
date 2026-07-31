use core::str::FromStr;
use heapless::String;
use nom::character::complete::char;
use crate::error::PmtkError;
use crate::traits::{Message, Response};
use crate::types::{DataField, DATA_FIELD_LEN};

pub struct TxtMsg(pub String<DATA_FIELD_LEN>);

impl Message for TxtMsg {
    const PKT_TYPE: u16 = 11;
}

impl TryFrom<DataField> for TxtMsg {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (msg, _) = char(',')(i)?;
        // TODO this is weird bc msg will be DATA_FIELD_LEN - 1 when parsing succeeds
        Ok(TxtMsg(String::from_str(msg)?))
    }
}

impl Response for TxtMsg {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::types::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",MTKGPS").unwrap();
        let txt_msg = TxtMsg::try_from(data_field).unwrap();
        assert_eq!(txt_msg.0, "MTKGPS");
    }
}
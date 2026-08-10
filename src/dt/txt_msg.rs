use core::str::FromStr;
use heapless::String;
use nom::character::complete::char;
use crate::error::PmtkError;
use crate::traits::{Packet, Response};
use crate::packet::{DataField, DATA_FIELD_LEN};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct TxtMsgDt(pub String<DATA_FIELD_LEN>);

impl Packet for TxtMsgDt {
    const PKT_TYPE: u16 = 11;
}

impl TryFrom<DataField> for TxtMsgDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (msg, _) = char(',')(i)?;
        // TODO this is weird bc msg will be DATA_FIELD_LEN - 1 when parsing succeeds
        Ok(TxtMsgDt(String::from_str(msg)?))
    }
}

impl Response for TxtMsgDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use crate::packet::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",MTKGPS").unwrap();
        let txt_msg = TxtMsgDt::try_from(data_field).unwrap();
        assert_eq!(txt_msg.0, "MTKGPS");
    }
}
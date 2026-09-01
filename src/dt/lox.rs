use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::{map_res, opt};
use nom::Parser;
use nom::multi::many0;
use nom::sequence::preceded;
use crate::error::PmtkError;
use crate::packet::{DataField, PktType};
use crate::parse;
use crate::parse::{number, number_in_range};
use crate::traits::{Dt, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Event {
    #[default]
    Start = 0x0,
    Data = 0x1,
    End = 0x2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LoxDt {
    blocks: Option<[u32; 24]>,
    n: Option<u16>,
    event: Event
}

impl Packet for LoxDt {
    const PKT_TYPE: PktType = [76, 79, 88]; // LOX
}

impl TryFrom<DataField> for LoxDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, e) = number_in_range::<u8>(i, 0, 2)?;

        if e == 2 {
            return Ok(LoxDt { blocks: None, n: None, event: Event::End });
        }

        let (i, _) = char(',').parse(i)?;
        let (i, n) = opt(|i| number::<u16>(i)).parse(i)?;

        if e == 0 {
            return Ok(LoxDt { blocks: None, n, event: Event::Start });
        }

        let (_, blocks_v) = many0(map_res(preceded(char(','), take(8usize)), parse::hex32)).parse(i)?;
        let blocks = if blocks_v.len() > 0 {
            let mut res: [u32; 24] = [0u32; 24];
            for n in 0..blocks_v.len() {
                res[n] = blocks_v[n];
            }
            Some(res)
        } else {
            None
        };
        Ok(LoxDt { blocks, n, event: Event::Data })
    }
}

impl Dt for LoxDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn locus_start_try_from_data_field_ok() {
        let data_field = DataField::from_str(",0,1366,").unwrap();
        let lox = LoxDt::try_from(data_field).unwrap();
        assert_eq!(lox.blocks, None);
        assert_eq!(lox.event, Event::Start);
        assert_eq!(lox.n, Some(1366));
    }

    #[test]
    fn locus_data_try_from_data_field_ok() {
        let data_field = DataField::from_str(",1,0,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF,FFFFFFFF").unwrap();
        let lox = LoxDt::try_from(data_field).unwrap();
        assert_eq!(lox.blocks, Some([4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295]));
        assert_eq!(lox.event, Event::Data);
        assert_eq!(lox.n, Some(0));
    }

    #[test]
    fn locus_end_try_from_data_field_ok() {
        let data_field = DataField::from_str(",2").unwrap();
        let lox = LoxDt::try_from(data_field).unwrap();
        assert_eq!(lox.blocks, None);
        assert_eq!(lox.event, Event::End);
        assert_eq!(lox.n, None);
    }
}
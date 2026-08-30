use nom::Parser;
use core::str::FromStr;
use heapless::String;
use nom::bytes::complete::{take, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::preceded;
use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket};

fn hex(data: &str) -> Result<u8, &'static str> {
    u8::from_str_radix(data, 16).map_err(|_| "Failed to parse checksum as hex number")
}

fn num<I: FromStr>(data: &str) -> Result<I, &'static str> {
    data.parse::<I>().map_err(|_| "parse of number failed")
}

fn checksum(i: &str) -> IResult<&str, u8> {
    map_res(preceded(char('*'), take(2usize)), hex).parse(i)
}

pub(crate) fn number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_res(digit1, num).parse(i)
}

pub(crate) fn number_in_range<T>(
    i: &str,
    lower_bound: T,
    upper_bound_inclusive: T,
) -> IResult<&str, T>
where
    T: PartialOrd + FromStr,
{
    map_res(number::<T>, |parsed_num| {
        if parsed_num < lower_bound || parsed_num > upper_bound_inclusive {
            return Err("Parsed number is outside of the expected range");
        }
        Ok(parsed_num)
    })
        .parse(i)
}

pub(crate) fn packet(i: &str) -> Result<PmtkPacket, PmtkError> {
    let (i, _) = talker_id(i)?;
    let (i, pkt_type) = packet_type(i)?;
    let (i, data_field) = take_until("*").parse(i)?;
    let (_, checksum) = checksum(i)?;

    PmtkPacket::new(
        pkt_type,
        if !data_field.is_empty() {
            Some(String::from_str(data_field).map_err(|_| PmtkError::Parsing)?)
        } else {
            None
        },
        Some(checksum),
    )
}

pub(crate) fn packet_type(i: &str) -> IResult<&str, PktType> {
    map_res(take(3usize), |packet_type: &str| {
        packet_type.as_bytes().try_into()
    }).parse(i)
}

fn talker_id(i: &str) -> IResult<&str, &str> {
    map_res(preceded(char('$'), take(4usize)), |talker_id| {
        if talker_id != "PMTK" { return Err("Invalid talker id") } else { Ok(talker_id) }
    }).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_type_numeric_ok() {
        let res = packet_type("419").unwrap();
        assert_eq!([52, 49, 57], res.1)
    }

    #[test]
    fn packet_type_char_ok() {
        let res = packet_type("LOG").unwrap();
        assert_eq!([76, 79, 71], res.1)
    }
}
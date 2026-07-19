use core::str::FromStr;
use heapless::{format, String};
use nom::bytes::complete::{take, take_until};
use nom::character::complete::{char, digit1};
use nom::{IResult, Parser};
use nom::combinator::map_res;
use nom::sequence::preceded;
use crate::error::PmtkError;

pub(crate) const PACKET_LEN: usize = 255;
pub(crate) const PAYLOAD_LEN: usize = 246;
pub(crate) const DATA_FIELD_LEN: usize = 242;

pub(crate) type DataField = String<DATA_FIELD_LEN>;

#[derive(Debug, PartialEq)]
pub struct PmtkPacket {
    pub(crate) checksum: u8,
    pub(crate) data_field: DataField,
    pub(crate) pkt_type: u16
}

impl PmtkPacket {
    pub fn new(data_field: DataField, pkt_type: u16) -> Self {
        let payload = format!(PAYLOAD_LEN; "PMTK{}{}", pkt_type, data_field).unwrap(); // TODO remove .unwrap()
        let checksum = generate_checksum(payload.as_bytes());
        Self { checksum, data_field, pkt_type }
    }

    pub fn decode(raw: &str) -> Result<Self, PmtkError> {
        parse_packet(raw)
    }

    pub fn encode(&self) -> String<PACKET_LEN> {
        format!(PACKET_LEN; "$PMTK{}{}*{:X?}\r\n", self.pkt_type, self.data_field, self.checksum).unwrap() // TODO remove .unwrap()
    }
}

fn generate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &x| acc ^ x)
}

// parsing.rs

pub(crate) fn number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_res(digit1, parse_num).parse(i)
}

fn parse_packet(i: &str) -> Result<PmtkPacket, PmtkError> {
    let (i, _) = parse_talker_id(i)?;
    let (i, pkt_type) = parse_packet_type(i)?;
    //let (i, _) = char(',').parse(i)?;
    let (i, data_field) = take_until("*").parse(i)?;
    let (_, checksum) = parse_checksum(i)?;

    // TODO validate checksum?

    Ok(
        PmtkPacket {
            pkt_type,
            data_field: String::from_str(data_field).unwrap(), // TODO remove unwrap()
            checksum,
        }
    )
}
fn parse_checksum(i: &str) -> IResult<&str, u8> {
    map_res(preceded(char('*'), take(2usize)), parse_hex).parse(i)
}

fn parse_hex(data: &str) -> Result<u8, &'static str> {
    u8::from_str_radix(data, 16).map_err(|_| "Failed to parse checksum as hex number")
}

fn parse_num<I: FromStr>(data: &str) -> Result<I, &'static str> {
    data.parse::<I>().map_err(|_| "parse of number failed")
}

pub(crate) fn parse_number_in_range<T>(
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

fn parse_packet_type(i: &str) -> IResult<&str, u16> {
    map_res(take(3usize), |packet_type: &str| {
        u16::from_str_radix(packet_type, 10).map_err(|_| PmtkError::Parsing)
    }).parse(i)
}

fn parse_talker_id(i: &str) -> IResult<&str, &str> {
    map_res(preceded(char('$'), take(4usize)), |talker_id| {
        if talker_id != "PMTK" { return Err("Invalid talker id") } else { Ok(talker_id) }
    }).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_ok() {
        assert_eq!(
            PmtkPacket { checksum: 31, data_field: String::from_str(",1000").unwrap(), pkt_type: 220 },
            PmtkPacket::decode("$PMTK220,1000*1F\r\n").unwrap()
        );
    }

    #[test]
    fn encode_ok() {
        assert_eq!(
            "$PMTK220,1000*1F\r\n",
            PmtkPacket::new(String::from_str(",1000").unwrap(), 220).encode()
        );
    }

    #[test]
    fn generate_checksum_ok() {
        assert_eq!(generate_checksum(b"PMTK011,MTKGPS"), 08);
    }
}

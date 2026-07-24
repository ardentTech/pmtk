use nom::Parser;
use core::str::FromStr;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

fn parse_num<I: FromStr>(data: &str) -> Result<I, &'static str> {
    data.parse::<I>().map_err(|_| "parse of number failed")
}

pub(crate) fn number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_res(digit1, parse_num).parse(i)
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
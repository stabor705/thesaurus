use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::take;
use nom::character::complete::char;
use nom::combinator::{map, map_res};
use nom::multi::count;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;
use std::str;

const CRLF: &[u8] = b"\r\n";
const ERR_PREFIX: &[u8] = b"-ERR ";

#[derive(Debug, PartialEq)]
enum RespValue<'a> {
    String(&'a [u8]),
    Integer(i64),
    Error(&'a [u8]),
    Array(Vec<RespValue<'a>>),
}

fn resp(input: &[u8]) -> IResult<&[u8], RespValue> {
    alt((
        map(string, RespValue::String),
        map(integer, RespValue::Integer),
        map(error, RespValue::Error),
        map(bulk_string, RespValue::String),
        map(array, RespValue::Array),
    ))
    .parse(input)
}

fn line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until(CRLF), tag(CRLF)).parse(input)
}

fn string(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(char('+'), line).parse(input)
}

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    map_res(preceded(char(':'), line), |s: &[u8]| {
        str::from_utf8(s).unwrap().parse::<i64>()
    })
    .parse(input)
}

fn error(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(tag(ERR_PREFIX), line).parse(input)
}

fn bulk_string(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = bulk_string_length_prefix(input)?;
    terminated(take(length), tag(CRLF)).parse(input)
}

fn bulk_string_length_prefix(input: &[u8]) -> IResult<&[u8], usize> {
    map_res(preceded(char('$'), line), |s: &[u8]| {
        str::from_utf8(s).unwrap().parse::<usize>()
    })
    .parse(input)
}

fn array(input: &[u8]) -> IResult<&[u8], Vec<RespValue>> {
    let (input, length) = array_length_prefix(input)?;
    count(resp, length).parse(input)
}

fn array_length_prefix(input: &[u8]) -> IResult<&[u8], usize> {
    map_res(preceded(char('*'), line), |s: &[u8]| {
        str::from_utf8(s).unwrap().parse::<usize>()
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = b"Hello, World!\r\n";
        let result = line(input);
        assert_eq!(result.unwrap().1, b"Hello, World!");
    }

    #[test]
    fn test_parse_string() {
        let input = b"+OK\r\n";
        let result = resp(input);
        assert_eq!(result.unwrap().1, RespValue::String(b"OK"));
    }

    #[test]
    fn test_parse_integer() {
        let input = b":12345\r\n";
        let result = resp(input);
        assert_eq!(result.unwrap().1, RespValue::Integer(12345));
    }

    #[test]
    fn test_parse_error() {
        let input = b"-ERR some error message\r\n";
        let result = resp(input);
        assert_eq!(result.unwrap().1, RespValue::Error(b"some error message"));
    }

    #[test]
    fn test_parse_length_prefix() {
        let input = b"$6\r\n";
        let result = bulk_string_length_prefix(input);
        assert_eq!(result.unwrap().1, 6);
    }

    #[test]
    fn test_parse_bulk_string() {
        let input = b"$12\r\nHello World!\r\n";
        let result = resp(input);
        assert_eq!(result.unwrap().1, RespValue::String(b"Hello World!"));
    }

    #[test]
    fn test_parse_array() {
        let input = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let result = resp(input);
        assert_eq!(
            result.unwrap().1,
            RespValue::Array(vec![RespValue::String(b"foo"), RespValue::String(b"bar")])
        );
    }
}

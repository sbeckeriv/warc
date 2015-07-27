use nom::{IResult,not_line_ending, space, alphanumeric, multispace, digit, Needed, Err};
use nom::IResult::*;
// Parser definition
use std::str;
use std::str::FromStr;
fn version_number(input: &[u8]) -> IResult<&[u8], &[u8]> {
    for (idx, chr) in input.iter().enumerate() {
        match *chr {
            46 | 48...57  => continue,
            _ => return IResult::Done(&input[idx..], &input[..idx]),
        }
    }
    IResult::Incomplete(Needed::Size(1))
}
fn token(input: &[u8]) -> IResult<&[u8], &[u8]> {
    for (idx, chr) in input.iter().enumerate() {
        match *chr {
            33 | 35...39 | 42 | 43 | 45 | 48...57 | 65...90 | 94...122 | 124 => continue,
            _ => return IResult::Done(&input[idx..], &input[..idx]),
        }
    }
    IResult::Incomplete(Needed::Size(1))
}
named!(pub init_line <&[u8], (&[u8])>,
dbg!(chain!(
        tag!("WARC")                ~
        tag!("/")                   ~
        space?                      ~
        version: version_number     ~
        tag!("\r")?                 ~
        tag!("\n")                  ,
        || {(version)}
        )
    ));
named!(pub header_match <&[u8], (&[u8], &[u8])>,
dbg!(chain!(
        name: token                 ~
        space?                      ~
        tag!(":")                   ~
        space?                      ~
        value: token                ~
        tag!("\r")?                 ~
        tag!("\n")                  ,
        || {(name, value)}
        )
    ));
named!(pub header_aggregator<&[u8], Vec<(&[u8],&[u8])> >, many1!(header_match));
named!(pub warc_header<&[u8], ((&[u8]), Vec<(&[u8],&[u8])>) >,
chain!(
    version: init_line              ~
    headers: header_aggregator      ,
    move ||{(version, headers)}
    )
);

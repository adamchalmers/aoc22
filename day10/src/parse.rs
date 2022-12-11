use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64 as register_value, line_ending},
    combinator::{map, value},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

use crate::Instruction;

type Input<'a> = &'a [u8];

impl Instruction {
    pub fn parse_input(i: Input) -> IResult<Input, Vec<Self>> {
        separated_list0(line_ending, Self::parse_one)(i)
    }
    fn parse_one(i: Input) -> IResult<Input, Self> {
        alt((Self::parse_noop, Self::parse_addx))(i)
    }
    fn parse_noop(i: Input) -> IResult<Input, Self> {
        value(Self::Noop, tag("noop"))(i)
    }
    fn parse_addx(i: Input) -> IResult<Input, Self> {
        map(preceded(tag("addx "), register_value), Self::Addx)(i)
    }
}

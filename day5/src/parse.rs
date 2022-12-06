use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char as onechar, line_ending, u32 as parse_u32},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use super::{Rearrangement, Row};

/// The input to each parser is a slice of bytes.
type Input<'a> = &'a [u8];

/// The result from parsing an input.
///  OK branch is (T, remainder of bytes)
/// Err branch is a nom error.
type Result<'a, T> = IResult<&'a [u8], T>;

pub fn entire_input(input: &[u8]) -> (Row, Vec<Rearrangement>) {
    fn ignored_line(i: Input) -> Result<()> {
        value((), terminated(take_until("\n"), line_ending))(i)
    }
    map(
        tuple((
            initial_stacked_rows,
            ignored_line,
            ignored_line,
            rearrangements,
        )),
        |(row, _, _, rearrangements)| (row, rearrangements),
    )(input)
    .expect("parse error")
    .1
}

/// Parse the input describing the initial arrangement of stacked crates.
fn initial_stacked_rows(i: Input) -> Result<Row> {
    /// Parse one line of the input, describing an initial row of the crates.
    fn row(i: Input) -> Result<Row> {
        let p = separated_list1(onechar(' '), alt((parse_crate, parse_no_crate)));
        map(p, Row)(i)
    }
    /// Parse a column with one crate.
    fn parse_crate(i: Input) -> Result<Vec<char>> {
        let p = delimited(onechar('['), anychar, onechar(']'));
        map(p, |ch| vec![ch])(i)
    }
    /// Parse a column with no crates.
    fn parse_no_crate(i: Input) -> Result<Vec<char>> {
        // Note `value(x, p)` === `map(p, |_out| x)`
        // Basically `value` is sugar for `map` where you don't need the output value.
        value(Vec::new(), tag("   "))(i)
    }

    /// Parse and stack all rows into one row with many columns of crates.
    fn all_rows(i: Input) -> Result<Row> {
        map(separated_list1(line_ending, row), |rows| {
            rows.into_iter()
                .reduce(|previous_rows, row| row.stack(previous_rows))
                .expect("must have > 0 rows")
        })(i)
    }

    terminated(all_rows, onechar('\n'))(i)
}

/// Parse all crane rearrangements.
fn rearrangements(i: Input) -> Result<Vec<Rearrangement>> {
    let parse_rearrangement = map(
        tuple((
            tag("move "),
            parse_u32,
            tag(" from "),
            parse_u32,
            tag(" to "),
            parse_u32,
        )),
        |(_, qty, _, src, _, dst)| Rearrangement {
            qty: qty as _,
            // Convert the input format's 1-based indices to 0-based indices
            src: (src - 1) as _,
            dst: (dst - 1) as _,
        },
    );
    separated_list1(line_ending, parse_rearrangement)(i)
}

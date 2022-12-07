use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1, take_while},
    character::{
        complete::{alpha1, char as one_char, line_ending, u32 as parse_num},
        is_alphabetic,
    },
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
};

type Input<'a> = &'a [u8];
type Result<'a, T> = nom::IResult<Input<'a>, T>;

#[derive(Clone, Debug)]
pub enum Line {
    Cd(String),
    LsFile(u32, String),
    Other,
}

/// Parse the entire input file.
pub fn parse_lines(i: Input) -> Result<Vec<Line>> {
    separated_list1(line_ending, alt((parse_cd, parse_ls_file, parse_other)))(i)
}

fn parse_cd(i: Input) -> Result<Line> {
    fn parse_dir_name(i: Input) -> Result<String> {
        map_res(alt((tag("/"), tag(".."), alpha1)), |bytes: &[u8]| {
            String::from_utf8(bytes.to_vec())
        })(i)
    }
    map(preceded(tag("$ cd "), parse_dir_name), Line::Cd)(i)
}

fn parse_other(i: Input) -> Result<Line> {
    value(Line::Other, take_until1("\n"))(i)
}

fn parse_ls_file(i: Input) -> Result<Line> {
    fn parse_file_name(i: Input) -> Result<String> {
        const PERIOD: u8 = 46;
        map_res(
            take_while(|c| is_alphabetic(c) || c == PERIOD),
            |bytes: &[u8]| String::from_utf8(bytes.to_vec()),
        )(i)
    }
    map(
        tuple((parse_num, one_char(' '), parse_file_name)),
        |(size, _, name)| Line::LsFile(size, name),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let input = include_bytes!("../example");
        let (_, lines) = parse_lines(input).unwrap();
        assert_eq!(lines.len(), 23);
    }

    #[test]
    fn test_parse_input() {
        let input = include_bytes!("../input");
        let (_, lines) = parse_lines(input).unwrap();
        assert_eq!(lines.len(), 950);
    }
}

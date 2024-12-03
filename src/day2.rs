use crate::util;

use winnow::{
    ascii::{digit1, line_ending, multispace1, newline, space0},
    combinator::{alt, repeat, separated, separated_pair, terminated},
    PResult, Parser,
};

pub fn day_2(part: u8) -> std::io::Result<i32> {
    println!("Day 2 part {}", part);

    let input = util::read_input(2, 1);

    if part == 1 {
        part_1(&input);
    }

    Ok(0)
}

fn part_1(input: &str) {}

fn parse_item(input: &mut &str) -> PResult<i32> {
    let s = digit1.parse_next(input)?;
    let val: i32 = s.parse().expect("not number");
    Ok(val)
}

fn parse_line_items(input: &mut &str) -> PResult<Vec<i32>> {
    terminated(separated(0.., parse_item, " "), newline).parse_next(input)
}

fn parse_lines(input: &mut &str) -> PResult<Vec<Vec<i32>>> {
    repeat(0.., parse_line_items).parse_next(input)
}

// fn parse_line(input: &mut &str) -> PResult<Vec<i32>> {
//     repeat(
//         1..,
//         (digit1.map(|s: &str| s.parse::<i32>().unwrap()), space0),
//     )
//     .parse_next(input)
// }

// fn parse_item(input: &mut &str) -> PResult<i32> {
//     let s = digit1.parse_next(input)?.try_map(|v: &str| v.parse::<i32>().expect("not num"))
// }

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_line() {
        let line = "12 23 34\n";

        let r = parse_line_items.parse(line).unwrap();

        let expected = vec![12, 23, 34];

        assert_eq!(expected, r);
    }

    #[test]
    fn can_parse_example() {
        let r = parse_lines.parse(example).unwrap();

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(expected, r);
    }
}

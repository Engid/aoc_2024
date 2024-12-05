use crate::util;

use winnow::{
    ascii::{digit1, line_ending, multispace1, newline, space0},
    combinator::{alt, delimited, eof, opt, repeat, separated, separated_pair, terminated},
    token::{literal, take, take_until, take_while},
    PResult, Parser,
};

pub fn solve(part: u8) -> std::io::Result<()> {
    println!("Day 3 part {}", part);

    let input = util::read_input(3, 1);

    if part == 1 {
        process_part_one(&input);
    } else if part == 2 {
        process_part_two(&input);
    } else {
        panic!("Invalid part");
    }

    Ok(())
}

fn process_part_one(input: &str) {}

fn process_part_two(input: &str) {}

fn parse_arg(input: &mut &str) -> PResult<i32> {
    digit1.map(|x: &str| x.parse().unwrap()).parse_next(input)
}

fn parse_mul_args(input: &mut &str) -> PResult<Option<(i32, i32)>> {
    let r = delimited("mul(", separated_pair(parse_arg, ",", parse_arg), ")").parse_next(input)?;
    Ok(Some(r))
}

fn parse_multiply(input: &mut &str) -> PResult<(i32, i32)> {
    delimited("mul(", separated_pair(parse_arg, ",", parse_arg), ")").parse_next(input)
}

fn match_multiply<'i>(input: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    delimited("mul(", separated_pair(digit1, ",", digit1), ")").parse_next(input)
}

fn parse_mul_arg_strs<'i>(input: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    delimited("mul(", separated_pair(digit1, ",", digit1), ")").parse_next(input)
}

fn take_until_mul<'i>(input: &mut &'i str) -> PResult<&'i str> {
    take_until(0.., "mul(").parse_next(input)
}

fn parse_garb_and_multiply(input: &mut &str) -> PResult<Option<(i32, i32)>> {
    let mut result = None;

    if let Ok(Some(_)) = opt(take_until_mul).parse_next(input) {
        if let Ok(Some(r)) = opt(parse_multiply).parse_next(input) {
            // kindof silly to unwrap only to wrap it back up
            return Ok(Some(r));
        } else {
            "mul(".parse_next(input)?;
            return Ok(None);
        }
    }

    Ok(result)
}

fn parse_garbage<'i>(input: &mut &'i str) -> PResult<Option<(i32, i32)>> {
    let _ = take_until(0.., "mul(").parse_next(input)?;
    Ok(None)
}

fn parse_mul_ignore_garbage(input: &mut &str) -> PResult<Option<(i32, i32)>> {
    let r = alt((parse_garbage, parse_mul_args)).parse_next(input)?;
    Ok(r)
}

// fn parse_mul_ignore_garbage_alt(input: &mut &str) -> PResult<(i32, i32)> {
//     let result = alt((parse_garbage, parse_mul_arg_strs)).parse_next(input);
//     Ok(result)
// }

// fn parse(input: &mut &str) -> PResult<Vec<Option<(i32, i32)>>> {
//     take(0.., parse_mul_ignore_garbage).parse_next(input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    // produces 161 (2*4 + 5*5 + 11*8 + 8*5)
    const example: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn this_good() {
        let mut input = example;

        let mut results: Vec<(i32, i32)> = Vec::new();

        while let Ok(res) = parse_garb_and_multiply.parse_next(&mut input) {
            if let Some(args) = res {
                results.push(args);
            }
        }

        let expected = vec![(2, 4), (3, 7), (11, 8), (8, 5)];

        assert_eq!(expected, results);
    }

    #[test]
    fn bad_input() {
        let mut input = "+mul(32,64]";

        let result = parse_garb_and_multiply.parse_next(&mut input);

        assert_eq!(None, result.unwrap());
    }

    // #[test]
    // fn test_parse() {
    //     let result = parse.parse(&example).unwrap();
    //     let expected = vec![Some((2, 4)), Some((5, 5)), Some((11, 8)), Some((8, 5))];
    //     assert_eq!(expected, result)
    // }

    #[test]
    fn test_parse_mul() {
        let mut e = "mul(2,4)";
        let result = parse_mul_args.parse(e).unwrap();
        let expected = Some((2, 4));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_parse_mul_full() {
        let mut e = "xmul(2,4)%&mul[3,7]!";
        let pre_garbage = parse_garbage.parse_next(&mut e).unwrap();
        let result = parse_mul_args.parse_next(&mut e).unwrap();

        assert_eq!(None, pre_garbage);
        assert_eq!(Some((2, 4)), result);
        assert_eq!("%&mul[3,7]!", e);
    }

    #[test]
    fn test_parse_mul_ignore_garbage() {
        let mut e = "xmul(2,4)%&mul[3,7]!";

        let result = parse_mul_ignore_garbage.parse_next(&mut e).unwrap();

        let result2 = parse_mul_ignore_garbage.parse_next(&mut e).unwrap();

        assert_eq!(Some((2, 4)), result);
        assert_eq!("%&mul[3,7]!", e);
    }

    // #[test]
    // fn chat_cray() {
    //     let mut input = "abc123def456ghi789";
    //     let digit_parser = take_while(1.., digit1);
    //     let ignore_parser = take_until(0.., digit1);

    //     // Combine parsers: attempt to parse digits; if not, consume and ignore non-matching input
    //     let combined_parser = alt((digit_parser.map(Some), ignore_parser.map(|_| None)));

    //     // Apply the combined parser repeatedly over the input
    //     let parser = repeat(0.., combined_parser);

    //     match parser.parse_next(&mut input) {
    //         Ok(matches) => {
    //             let digits: Vec<_> = matches.into_iter().flatten().collect();
    //             println!("Matched digit sequences: {:?}", digits);
    //         }
    //         Err(err) => println!("Error: {:?}", err),
    //     }
    // }
}

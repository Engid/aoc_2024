use std::env;
use std::fs::File;
use std::io::Read;

use winnow::{
    ascii::{digit1, multispace1, newline},
    combinator::{repeat, separated_pair, terminated},
    PResult, Parser,
};

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("input/day1/input1.txt");
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    //println!("{}", contents);

    let answer = process_input(contents).unwrap();

    println!("{}", answer);

    Ok(())
}

fn process_input(input: String) -> std::io::Result<i32> {
    let lines_result = parse_lines.parse(&input);

    let lines = match lines_result {
        Ok(ls) => ls,
        Err(message) => panic!("{}", message),
    };

    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = lines.into_iter().unzip();

    first_list.sort();
    second_list.sort();

    let sum: i32 = first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| if a < b { b - a } else { a - b })
        .sum();

    Ok(sum)
}

fn parse_lines(input: &mut &str) -> PResult<Vec<(i32, i32)>> {
    repeat(0.., parse_items).parse_next(input)
}

fn parse_item(input: &mut &str) -> PResult<i32> {
    let s = digit1.parse_next(input)?;
    let val: i32 = s.parse().expect("not number");
    Ok(val)
}

fn parse_items(input: &mut &str) -> PResult<(i32, i32)> {
    terminated(separated_pair(parse_item, multispace1, parse_item), newline).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
" // note: needs new line for this parser...
        .to_owned();
        let result = parse_lines.parse(&input).unwrap();
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        assert_eq!(expected, result)
    }

    #[test]
    fn test_line() {
        let t = "12345   23456\n";
        let r = parse_items.parse(t).unwrap();
        let expected = (12345, 23456);
        assert_eq!(expected, r)
    }

    #[test]
    fn test_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
"
        .to_owned();

        let r = process_input(input).unwrap_or_default();

        assert_eq!(11, r);
    }
}

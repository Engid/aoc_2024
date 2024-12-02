use itertools::*;
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

    let input = contents.as_str();

    let answer1 = process_part_one(input).unwrap();

    println!("answer 1 {}", answer1);

    let answer2 = process_part_two(input).unwrap();

    println!("answer 2 {}", answer2);

    Ok(())
}

fn process_part_one(input: &str) -> std::io::Result<i32> {
    let lines_result = parse_lines.parse(input);

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

fn process_part_two(input: &str) -> std::io::Result<i64> {
    let lines_result = parse_lines.parse(&input);

    let lines = match lines_result {
        Ok(ls) => ls,
        Err(message) => panic!("{}", message),
    };

    let (first_list, second_list): (Vec<i32>, Vec<i32>) = lines.into_iter().unzip();

    // TODO: take second list, do a count of each item, turn into hashmap
    //      loop through second list, and do the math
    // let first_list_counts = first_list
    //     .into_iter()
    //     .into_grouping_map_by(|key| *key)
    //     .fold(0, |acc, _key, _| acc + 1);

    let second_list_counts = second_list
        .into_iter()
        .into_grouping_map_by(|key| *key)
        .fold(0, |acc, _key, _| acc + 1);

    let mut total = 0_i64;

    for k in first_list {
        if let Some(second_list_count) = second_list_counts.get(&k) {
            println!("k {}, count {}", k, second_list_count);
            let first_list_num = k as i64;
            //let times_first_list = v as i64;
            let times_second_list = *second_list_count as i64;
            //let key_times = first_list_num * times_first_list;
            let second_list_times = first_list_num * times_second_list;
            total += second_list_times;
        }
    }

    // let sum: i32 = first_list
    //     .iter()
    //     .zip(second_list.iter())
    //     .map(|(a, b)| if a < b { b - a } else { a - b })
    //     .sum();

    Ok(total)
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
";
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
";

        let r = process_part_one(&input).unwrap_or_default();

        assert_eq!(11, r);
    }

    #[test]
    fn test_part_two() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";

        let r = process_part_two(&input).unwrap_or_default();

        assert_eq!(31, r);
    }

    #[test]
    fn itter_test() {
        let items = vec![
            ("apple", 10),
            ("banana", 5),
            ("apple", 3),
            ("banana", 2),
            ("orange", 7),
        ];

        let map = items
            .into_iter()
            .into_grouping_map_by(|(key, _)| *key)
            .fold(0, |acc, _, (_, value)| acc + value);

        println!("{:?}", map);
    }
}

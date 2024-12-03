use core::panic;

use crate::util;

use winnow::{
    ascii::{digit1, line_ending, multispace1, newline, space0},
    combinator::{alt, repeat, separated, separated_pair, terminated},
    PResult, Parser,
};

pub fn solve(part: u8) -> std::io::Result<i32> {
    println!("Day 2 part {}", part);

    let input = util::read_input(2, 1);

    if part == 1 {
        get_saftey_report(&input, false);
    } else if part == 2 {
        get_saftey_report(&input, true);
    } else {
        panic!("Invalid part");
    }

    Ok(0)
}

enum Direction {
    Up,
    Down,
}

fn get_saftey_report(input: &str, dampen: bool) -> usize {
    let readings = parse_lines.parse(input).unwrap();

    let mut unsafe_counts: Vec<i32> = Vec::new();

    for levels in readings {
        let mut direction: Option<Direction> = None;
        let mut last_num: i32 = 0;
        //let mut unsafe_count: i32 = 0;
        let mut level_safety: Vec<(i32, bool)> = Vec::new();

        for (idx, level) in levels.iter().enumerate() {
            if idx == 0 {
                last_num = *level;
                continue;
            }

            let diff = level - last_num;

            if diff.abs() > 3 || diff == 0 {
                level_safety.push((*level, false));
                continue;
            }

            // check direction
            match diff {
                // negative
                x if x < 0 => match direction {
                    Some(Direction::Up) => {
                        level_safety.push((*level, false));
                        continue;
                    }
                    None => {
                        direction = Some(Direction::Down);
                    }
                    _ => {}
                },
                // redundant but ok
                0 => {
                    level_safety.push((*level, false));
                    continue;
                }
                // positive
                _ => match direction {
                    Some(Direction::Down) => {
                        level_safety.push((*level, false));
                        continue;
                    }
                    None => {
                        direction = Some(Direction::Up);
                    }
                    _ => {}
                },
            }
            last_num = *level;
            level_safety.push((*level, true));
        }

        //unsafe_counts.push(unsafe_count);

        let unsafe_count = level_safety.iter().filter(|&(_, safe)| !safe).count() as i32;

        if unsafe_count > 1 {
            // permutate different ways to remove a single unsafe level
            let mut new_levels: Vec<Vec<i32>> = Vec::new();

            // make copies to then filter out one level from
            for _ in 0..unsafe_count {
                new_levels.push(levels.clone());
            }
        } else if dampen && unsafe_count == 1 {
            // we can just remove it and mark it safe
            unsafe_counts.push(0);
        }

        direction = None;
        last_num = 0;
        level_safety = Vec::new();
    }

    let safe_count = unsafe_counts
        .iter()
        .filter(|&s| if dampen { *s <= 1 } else { *s <= 0 })
        .count();

    println!("Safe count: {}", safe_count);

    safe_count
}

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
    fn test_part_one() {
        let safe_count = get_saftey_report(example, false);

        assert_eq!(2, safe_count);
    }

    #[test]
    fn test_part_two() {
        let safe_count = get_saftey_report(example, true);

        assert_eq!(4, safe_count);
    }

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

    #[test]
    fn split_example() {
        // realized that writing a parser is a little overkill,
        // so wanted to compare with simply splitting the string by newlines and spaces..

        // this is pretty simple.. a little more code tho

        let lines: Vec<&str> = example.split("\n").collect();

        let numbers: Vec<Vec<i32>> = lines
            .iter()
            .filter(|&l| !l.is_empty())
            .map(|&l| {
                l.split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(expected, numbers);
    }
}

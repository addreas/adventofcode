use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn neighbors(xs: (usize, usize), y: usize) -> Vec<(usize, usize)> {
    let ymin = match y {
        0 => 0,
        _ => y - 1,
    };
    let xmin = match xs.0 {
        0 => 0,
        _ => xs.0 - 1,
    };
    let mut res = vec![];
    for y in vec![ymin, y, y + 1] {
        for x in xmin..(xs.1 + 1) {
            res.push((x, y));
        }
    }
    res
}

fn do_it(input: &str) -> u32 {
    let (parts, symbols) = parse(input);
    parts
        .into_iter()
        .filter(|&(xs, y, _)| {
            neighbors(xs, y)
                .iter()
                .any(|neigh| symbols.contains_key(neigh))
        })
        .map(|(_, _, num)| num)
        .sum()
}

fn do_it_again(input: &str) -> u32 {
    let (parts, symbols) = parse(input);

    let mut parts_index = HashMap::new();
    for (xs, y, num) in parts {
        for x in xs.0..xs.1 {
            parts_index.insert((x, y), num);
        }
    }

    symbols
        .into_iter()
        .filter(|&(_, char)| char == '*')
        .map(|((x, y), _)| {
            neighbors((x, x+1), y)
                .iter()
                .filter_map(|k| parts_index.get(k))
                .collect::<HashSet<_>>()
        })
        .filter(|set| set.len() == 2)
        .map(|set| set.into_iter().product::<u32>())
        .sum()
}

fn parse(
    input: &str,
) -> (
    Vec<((usize, usize), usize, u32)>,
    HashMap<(usize, usize), char>,
) {
    let mut symbols = HashMap::new();
    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut parts = vec![];

            let mut current_number = None;
            for (x, char) in line.chars().enumerate() {
                current_number = match current_number {
                    Some((x1, _, num)) if char.is_numeric() => {
                        Some((x1, x, 10 * num + char.to_digit(10).unwrap()))
                    }
                    Some((x1, _, num)) => {
                        parts.push(((x1, x), y, num));

                        if char != '.' {
                            symbols.insert((x, y), char);
                        }

                        None
                    }

                    None if char.is_numeric() => Some((x, x, char.to_digit(10).unwrap())),
                    None => {
                        if char != '.' {
                            symbols.insert((x, y), char);
                        }

                        None
                    }
                };
            }

            if let Some((x1, x2, num)) = current_number {
                parts.push(((x1, x2), y, num))
            }

            parts
        })
        .collect();

    (parts, symbols)
}

#[test]
fn test() {
    let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 4361);
    assert_eq!(do_it_again(input.as_str()), 467835)
}

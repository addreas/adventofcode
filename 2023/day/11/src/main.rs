use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref(), 1));
    println!("{}", do_it(input.as_ref(), 1000000-1));
}

fn count_smaller(set: &BTreeSet<usize>, item: usize) -> usize {
    for (i, &set_item) in set.iter().enumerate() {
        if set_item >= item {
            return i;
        }
    }
    set.len()
}

fn expand(galaxies: Vec<(usize, usize)>, times: usize) -> Vec<(usize, usize)> {
    let xs: BTreeSet<_> = galaxies.iter().map(|&(x, _)| x).collect();
    let ys: BTreeSet<_> = galaxies.iter().map(|&(_, y)| y).collect();

    let xmax = *xs.iter().max().unwrap();
    let ymax = *ys.iter().max().unwrap();

    let xs_to_double = (0..xmax)
        .collect::<BTreeSet<usize>>()
        .difference(&xs)
        .cloned()
        .collect();
    let ys_to_double = (0..ymax)
        .collect::<BTreeSet<usize>>()
        .difference(&ys)
        .cloned()
        .collect();

    galaxies
        .into_iter()
        .map(|(x, y)| {
            (
                x + times * count_smaller(&xs_to_double, x),
                y + times * count_smaller(&ys_to_double, y),
            )
        })
        .collect()
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    diff(a.0, b.0) + diff(a.1, b.1)
}

fn do_it(input: &str, times: usize) -> usize {
    let galaxies = parse(input);
    let expanded = expand(galaxies, times);

    println!("{expanded:?}");

    expanded
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| distance(a, b))
        .sum()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None,
            })
        })
        .collect()
}

#[test]
fn test() {
    let input = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str(), 1), 374);

    assert_eq!(do_it(input.as_str(), 10-1), 1030);
    assert_eq!(do_it(input.as_str(), 100-1), 8410);
}

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn do_it(input: &str) -> usize {
    let elements = parse(input);
    let xmax = *elements.keys().map(|(x, _)| x).max().unwrap();
    let ymax = *elements.keys().map(|(_, y)| y).max().unwrap();

    let mut q = vec![((0, 0), Dir::Right)]; // if (0,0) has an element it is never evaluated.

    let mut visited = HashSet::new();
    while let Some(((x, y), dir)) = q.pop() {
        println!("{x}, {y}, {dir:?}");

        if visited.contains(&((x, y), dir)) {

            continue;
        }

        visited.insert(((x, y), dir));

        if (x == 0 && dir == Dir::Left)
            || (y == 0 && dir == Dir::Up)
            || (x == xmax && dir == Dir::Right)
            || (y == ymax && dir == Dir::Down)
        {

            continue;
        }

        let next = match dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };

        match elements.get(&next) {
            Some('|') if dir == Dir::Left || dir == Dir::Right => {
                q.push((next, Dir::Up));
                q.push((next, Dir::Down));
            }
            Some('-') if dir == Dir::Up || dir == Dir::Down => {
                q.push((next, Dir::Left));
                q.push((next, Dir::Right));
            }
            Some('/') if dir == Dir::Up => q.push((next, Dir::Right)),
            Some('/') if dir == Dir::Down => q.push((next, Dir::Left)),
            Some('/') if dir == Dir::Left => q.push((next, Dir::Down)),
            Some('/') if dir == Dir::Right => q.push((next, Dir::Up)),

            Some('\\') if dir == Dir::Up => q.push((next, Dir::Left)),
            Some('\\') if dir == Dir::Down => q.push((next, Dir::Right)),
            Some('\\') if dir == Dir::Left => q.push((next, Dir::Up)),
            Some('\\') if dir == Dir::Right => q.push((next, Dir::Down)),

            _ => q.push((next, dir)),
        }
    }

    visited.iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len()
}

fn do_it_again(input: &str) -> usize {
    let elements = parse(input);

    0
}

fn parse(input: &str) -> HashMap<(usize, usize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                c => Some(((x, y), c)),
            })
        })
        .collect()
}
#[test]
fn test() {
    let input = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 46);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

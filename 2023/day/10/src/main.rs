fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

#[derive(Clone, Copy)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,

    Ground,

    Start,
}

fn first_step(map: &Vec<Vec<Pipe>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        match map[y - 1][x] {
            Pipe::NorthSouth => Some((x, y - 1)),
            Pipe::NorthEast => Some((x, y - 1)),
            Pipe::NorthWest => Some((x, y - 1)),
            _ => None,
        },
        match map[y + 1][x] {
            Pipe::NorthSouth => Some((x, y + 1)),
            Pipe::SouthEast => Some((x, y + 1)),
            Pipe::SouthWest => Some((x, y + 1)),
            _ => None,
        },
        match map[y][x - 1] {
            Pipe::EastWest => Some((x - 1, y)),
            Pipe::SouthEast => Some((x - 1, y)),
            Pipe::NorthEast => Some((x - 1, y)),
            _ => None,
        },
        match map[y][x + 1] {
            Pipe::EastWest => Some((x + 1, y)),
            Pipe::SouthWest => Some((x + 1, y)),
            Pipe::NorthWest => Some((x + 1, y)),
            _ => None,
        },
    ]
    .iter()
    .filter_map(|i| *i)
    .collect()
}

fn follow(map: &Vec<Vec<Pipe>>, start: (usize, usize), first: (usize, usize)) -> Option<usize> {
    let mut prev = start;
    let mut current = first;
    for i in 1.. {
        let (x, y) = current;
        let (left, right) = match map[current.1][current.0] {
            Pipe::NorthSouth => ((x, y - 1), (x, y + 1)),
            Pipe::EastWest => ((x + 1, y), (x - 1, y)),
            Pipe::NorthEast => ((x, y - 1), (x + 1, y)),
            Pipe::NorthWest => ((x, y - 1), (x - 1, y)),
            Pipe::SouthWest => ((x, y + 1), (x - 1, y)),
            Pipe::SouthEast => ((x, y + 1), (x + 1, y)),

            Pipe::Ground => return None,
            Pipe::Start => return Some(i),
        };

        if left == prev {
            prev = current;
            current = right;
        } else {
            prev = current;
            current = left;
        }
    }
    None
}

fn do_it(input: &str) -> usize {
    let (map, (x, y)) = parse(input);

    let start = (x, y);

    for first in first_step(&map, x, y) {
        if let Some(len) = follow(&map, start, first) {
            return len / 2;
        }
    }
    0
}

fn do_it_again(input: &str) -> usize {
    let map = parse(input);

    0
}

fn parse(input: &str) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut start = None;
    (
        input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '|' => Pipe::NorthSouth,
                        '-' => Pipe::EastWest,
                        'L' => Pipe::NorthEast,
                        'J' => Pipe::NorthWest,
                        '7' => Pipe::SouthWest,
                        'F' => Pipe::SouthEast,
                        '.' => Pipe::Ground,
                        'S' => {
                            start = Some((x, y));
                            Pipe::Start
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start.unwrap(),
    )
}

#[test]
fn test() {
    let input = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 4);
    assert_eq!(do_it_again(input.as_str()), 1);

    let input2 = "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "
    .trim()
    .replace("        ", "");

assert_eq!(do_it(input2.as_str()), 8);

    let input3 = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it_again(input3.as_str()), 4);

    let input4 = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it_again(input4.as_str()), 10);
}

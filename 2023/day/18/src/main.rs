use std::collections::HashSet;


fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}


fn go(start: (isize, isize), dir: char, steps: isize) -> Vec<(isize, isize)> {
    let (x, y) = start;
    match dir {
        'R' => (x..(x + steps)).into_iter().map(|n| (n+1, y)).collect(),
        'L' => ((x - steps)..x).into_iter().map(|n| (n, y)).rev().collect(),
        'D' => (y..(y + steps)).into_iter().map(|n| (x, n+1)).collect(),
        'U' => ((y - steps)..y).into_iter().map(|n| (x, n)).rev().collect(),
        _ => unreachable!()
    }
}

fn find_domain<'a> (points: impl IntoIterator<Item = &'a (isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    for &(x, y) in points {
        if x < x_range.0 {
            x_range.0 = x;
        } else if x > x_range.1 {
            x_range.1 = x;
        }

        if y < y_range.0 {
            y_range.0 = y;
        } else if y > y_range.1 {
            y_range.1 = y;
        }
    }

    (x_range, y_range)
}

fn count_inside(visited: HashSet<(isize, isize)>) -> usize  {
    let (x_range, y_range) = find_domain(&visited);
    let mut total = 0;
    for y in y_range.0..(y_range.1 + 1) {
        let mut inside = false;
        for x in (x_range.0 - 1)..(x_range.1 + 1) {
            if visited.contains(&(x, y)) && !visited.contains(&(x-1, y)) {
                inside = !inside;
                total += 1;
                print!("#");
            } else if inside {
                total += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
        // println!("total at line {y}: {total}");
    }

    total
}

fn do_it(input: &str) -> usize {
    let steps = parse(input);

    let mut visited: HashSet<(isize, isize)> = Default::default();
    let mut pos = (0,0);
    visited.insert(pos);
    for (dir, len, _) in steps.iter() {
        let ext = go(pos, *dir, *len);
        println!("going {dir} for {len} steps. visited: {:?}", ext);
        visited.extend(ext.iter());
        pos = *(ext.last()).unwrap();
    }

    print_visited(&visited);
    println!();

    count_inside(visited)
}

fn do_it_again(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn parse(input: &str) -> Vec<(char, isize, (u8, u8, u8))> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            (
                parts[0].chars().nth(0).unwrap(),
                parts[1].parse().unwrap(),
                parse_hex(&parts[2][2..parts[2].len() - 1]),
            )
        })
        .collect()
}

fn parse_hex(input: &str) -> (u8, u8, u8) {
    (
        u8::from_str_radix(&input[0..2], 16).unwrap(),
        u8::from_str_radix(&input[2..4], 16).unwrap(),
        u8::from_str_radix(&input[4..6], 16).unwrap(),
    )
}

fn print_visited(visited: &HashSet<(isize, isize)>) {
    let (x_range, y_range) = find_domain(visited);
    for y in y_range.0..(y_range.1 + 1) {
        for x in x_range.0..(x_range.1 + 1) {
            if visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn test() {
    let input = r"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 62);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

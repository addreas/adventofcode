#![feature(iter_array_chunks)]
#![feature(let_chains)]
use std::collections::BTreeSet;
use std::fs;
extern crate termion;

type P = (i64, i64);

fn main() {
    let rocks = get_rocks(fs::read_to_string("input.txt").unwrap());
    let source = (500, 0);
    print(&rocks, &BTreeSet::new(), source, source);

    let resting_grains: BTreeSet<P> = simulate(&rocks, source);
    println!("resting_grains: {}", resting_grains.len());

    let resting_grains: BTreeSet<P> = simulate2(&rocks, source);
    println!("resting_grains: {}", resting_grains.len());
}

fn print(rocks: &BTreeSet<P>, resting: &BTreeSet<P>, falling: P, source: P) {
    let (min_x, max_x, max_y) = (
        rocks
            .iter()
            .min_by_key(|(x, _)| x)
            .map(|(x, _)| x)
            .unwrap()
            .to_owned(),
        rocks
            .iter()
            .max_by_key(|(x, _)| x)
            .map(|(x, _)| x)
            .unwrap()
            .to_owned(),
        rocks
            .iter()
            .max_by_key(|(_, y)| y)
            .map(|(_, y)| y)
            .unwrap()
            .to_owned() + 2,
    );

    let top_line = (min_x..=max_x)
        .map(|x| match x {
            x if x == source.0 => "+",
            _ => ".",
        })
        .collect::<String>();

    println!(
        "{}  0 {}",
        // termion::clear::All,
        termion::cursor::Goto(1, 1),
        top_line
    );

    let (rows, _) = termion::terminal_size().unwrap();

    // for y in 1..=80 {
    for y in 1..=max_y.min(rows.into()) {
        let line = (min_x..=max_x)
            .map(|x| {
                if rocks.contains(&(x, y)) {
                    "#"
                } else if falling == (x, y) {
                    "O"
                } else if resting.contains(&(x, y)) {
                    "o"
                } else {
                    "."
                }
            })
            .collect::<String>();

        println!("{y:3} {}", line)
    }
}

fn can_go(rocks: &BTreeSet<P>, resting: &BTreeSet<P>, falling: P, dx: i64) -> Option<P> {
    let next = (falling.0 + dx, falling.1 + 1);
    if !rocks.contains(&next) && !resting.contains(&next) {
        Some(next)
    } else {
        None
    }
}

fn simulate(rocks: &BTreeSet<P>, source: P) -> BTreeSet<P> {
    let max_y = rocks
        .iter()
        .max_by_key(|(_, y)| y)
        .map(|(_, y)| y)
        .unwrap()
        .to_owned();
    let mut resting: BTreeSet<P> = BTreeSet::new();

    let mut abyss = false;

    while !abyss {
        let mut falling = source;
        loop {
            // until it hits rock

            /*
                A unit of sand always falls down one step if possible.
                If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move
                    diagonally one step down and to the left.
                If that tile is blocked, the unit of sand attempts to instead move
                    diagonally one step down and to the right.

                Sand keeps moving as long as it is able to do so,
                    at each step trying to move down,
                    then down-left,
                    then down-right.

                If all three possible destinations are blocked,
                the unit of sand comes to rest and no longer moves,
                at which point the next unit of sand is created back at the source.
            */

            if let Some(down) = can_go(&rocks, &resting, falling, 0) {
                falling = down;
            } else if let Some(down_left) = can_go(&rocks, &resting, falling,  -1) {
                falling = down_left;
            } else if let Some(down_right) = can_go(&rocks, &resting, falling, 1) {
                falling = down_right;
            } else {
                resting.insert(falling);
                break;
            }

            if falling.1 >= max_y {
                abyss = true;
                break;
            }

            // print(&rocks, &resting, falling, source);
        }
    }

    return resting;
}

fn can_go2(rocks: &BTreeSet<P>, resting: &BTreeSet<P>, falling: P, dx: i64, floor: i64) -> Option<P> {
    let next = (falling.0 + dx, falling.1 + 1);
    if !rocks.contains(&next) && !resting.contains(&next) && next.1 != floor {
        Some(next)
    } else {
        None
    }
}

fn simulate2(rocks: &BTreeSet<P>, source: P) -> BTreeSet<P> {
    let floor = rocks
        .iter()
        .max_by_key(|(_, y)| y)
        .map(|(_, y)| y)
        .unwrap()
        .to_owned() + 2;
    let mut resting: BTreeSet<P> = BTreeSet::new();


    while !resting.contains(&source) {
        let mut falling = source;
        loop {
            if let Some(down) = can_go2(&rocks, &resting, falling, 0, floor) {
                falling = down;
            } else if let Some(down_left) = can_go2(&rocks, &resting, falling, -1, floor) {
                falling = down_left;
            } else if let Some(down_right) = can_go2(&rocks, &resting, falling, 1, floor) {
                falling = down_right;
            } else {
                resting.insert(falling);
                break;
            }

            // print(&rocks, &resting, falling, source);
        }
    }

    return resting;
}

fn get_rocks(input: String) -> BTreeSet<P> {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|a| {
                    let coords: Vec<i64> = a.split(",").map(|a| a.parse().unwrap()).collect();
                    if let [x, y] = coords[..] {
                        (x, y)
                    } else {
                        todo!("too many coordinates")
                    }
                })
                .collect::<Vec<P>>()
        })
        .flat_map(|path| {
            std::iter::zip(path.iter(), path[1..].iter())
                .flat_map(|(from, to)| {
                    let (range, dir) = match (from, to) {
                        ((fx, fy), (tx, ty)) if fx == tx && fy < ty => (*fy..=(*ty), 0),
                        ((fx, fy), (tx, ty)) if fy == ty && fx < tx => (*fx..=(*tx), 1),
                        ((fx, fy), (tx, ty)) if fx == tx && fy > ty => (*ty..=(*fy), 0),
                        ((fx, fy), (tx, ty)) if fy == ty && fx > tx => (*tx..=(*fx), 1),
                        (_, _) => todo!("oh no"),
                    };
                    println!("from: {from:?}, to: {to:?}, range: {range:?}");
                    range
                        .map(|i| if dir == 0 { (from.0, i) } else { (i, from.1) })
                        .collect::<Vec<P>>()
                })
                .collect::<Vec<P>>()
        })
        .collect()
}

#[test]
fn test() {
    let rocks = get_rocks("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string());

    println!("{rocks:?}");

    assert!(rocks.contains(&(498, 4)));
    assert!(rocks.contains(&(498, 5)));
    assert!(rocks.contains(&(498, 6)));
    assert!(rocks.contains(&(497, 6)));
    assert!(rocks.contains(&(496, 6)));

    let source = (500, 0);
    print(&rocks, &BTreeSet::new(), source, source);

    println!("{}", termion::clear::All);
    let resting_grains: BTreeSet<P> = simulate(&rocks, source);
    assert_eq!(resting_grains.len(), 24);
}

#[test]
fn test2() {
    let rocks = get_rocks("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string());

    let source = (500, 0);

    println!("{}", termion::clear::All);
    let resting_grains: BTreeSet<P> = simulate2(&rocks, source);
    assert_eq!(resting_grains.len(), 93);
}

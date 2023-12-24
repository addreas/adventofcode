use std::collections::HashSet;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

type Brick = ((usize, usize, usize), (usize, usize, usize));

type Bricks = Vec<Brick>;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSQUVXYZ";

fn brick_name(brick: &Brick) -> &str {
    let mut hasher = DefaultHasher::new();
    brick.hash(&mut hasher);
    let index = hasher.finish() as usize % ALPHABET.len();
    &ALPHABET[index..=index]
}

fn xy_overlap(
    &((xf1, yf1, _), (xf2, yf2, _)): &Brick,
    &((xr1, yr1, _), (xr2, yr2, _)): &Brick,
) -> bool {
    let f: HashSet<_> = (xf1..=xf2)
        .flat_map(|x| (yf1..=yf2).map(move |y| (x, y)))
        .collect();
    let r: HashSet<_> = (xr1..=xr2)
        .flat_map(|x| (yr1..=yr2).map(move |y| (x, y)))
        .collect();
    let res = !r.is_disjoint(&f);

    // println!("({xf1}, {yf1})~({xf2}, {yf2}) overlaps ({xr1}, {yr1})~({xr2}, {yr2})? {res}");
    res
}

fn fall(mut bricks: Bricks) -> Bricks {
    assert!(bricks.iter().all(|(a, b)| {
        (b.0 - a.0 > 0) ^ (b.1 - a.1 > 0) ^ (b.2 - a.2 > 0)
            || ((b.0 == a.0) && (b.1 == a.1) && (b.2 == a.2))
    }));

    bricks.sort_by_key(|((_, _, z1), (_, _, z2))| *z1.min(z2));

    let mut fallen = vec![];

    for brick in bricks {
        let ((x1, y1, z1), (x2, y2, z2)) = brick;
        if let Some(((_, _, zr1), (_, _, zr2))) =
            fallen.iter().rev().find(|&b| xy_overlap(&brick, b))
        {
            let dz = z1.min(z2) - zr1.max(zr2) - 1;
            // println!("falling: ({x1}, {y1})~({x2},{y2}) - {dz}");
            fallen.push(((x1, y1, z1 - dz), (x2, y2, z2 - dz)));
        } else {
            let dz = z1.max(z2) - z1.min(z2);
            // println!("floor: ({x1}, {y1})~({x2},{y2}) - {dz}");
            fallen.push(((x1, y1, 1), (x2, y2, 1 + dz)));
        }
    }

    fallen
}

fn get_bricks_resting_on(this_brick: &Brick, bricks: &Bricks) -> Bricks {
    let top_of_this = this_brick.0 .2.max(this_brick.1 .2);

    let mut bricks_resting_on_this = vec![];
    for b in bricks {
        let bottom_of_b = b.0 .2.min(b.1 .2);
        if bottom_of_b == (top_of_this + 1) && xy_overlap(this_brick, b) {
            println!(
                "{b:?} rests on {this_brick:?}. {} on {}",
                brick_name(b),
                brick_name(this_brick)
            );
            bricks_resting_on_this.push(*b);
        }
    }

    bricks_resting_on_this
}

fn get_bricks_holding(this_brick: &Brick, bricks: &Bricks) -> Bricks {
    let bottom_of_this = this_brick.0 .2.min(this_brick.1 .2);

    let mut bricks_holding_this = vec![];
    for b in bricks {
        let top_of_b = b.0 .2.max(b.1 .2);
        if top_of_b == (bottom_of_this - 1) && xy_overlap(this_brick, b) {
            println!(
                "{b:?} holds up {this_brick:?}. {} on {}",
                brick_name(this_brick),
                brick_name(b)
            );
            bricks_holding_this.push(*b);
        }
    }

    bricks_holding_this
}

fn can_disintegrate(this_brick: &Brick, bricks: &Bricks) -> bool {
    get_bricks_resting_on(this_brick, bricks)
        .iter()
        .all(|b| get_bricks_holding(b, bricks).len() > 1)
}

fn do_it(input: &str) -> usize {
    let bricks = fall(parse(input));
    bricks
        .iter()
        .filter(|&b| can_disintegrate(b, &bricks))
        .count()
}

fn do_it_again(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn parse(input: &str) -> Bricks {
    input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once("~").unwrap();
            let fp: Vec<_> = from.split(",").map(|n| n.parse().unwrap()).collect();
            let tp: Vec<_> = to.split(",").map(|n| n.parse().unwrap()).collect();
            ((fp[0], fp[1], fp[2]), (tp[0], tp[1], tp[2]))
        })
        .collect()
}

#[test]
fn test() {
    let input = r"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 5);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

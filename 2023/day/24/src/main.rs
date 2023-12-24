use itertools::Itertools;

fn main() {
    let input = include_str!("../input");
    println!(
        "{}",
        do_it(input.as_ref(), 200000000000000, 400000000000000)
    );
    println!("{}", do_it_again(input.as_ref()));
}

type Particle = ((usize, usize, usize), (isize, isize, isize));

fn find_intersection(a: Particle, b: Particle) -> (f64, f64) {
    let ((ax, ay, az), (adx, ady, adz)) = a;
    let ((bx, by, bz), (bdx, bdy, bdz)) = b;

    // y_a = ax + c
    // y_b = bx + d

    let a = (ady as f64) / (adx as f64);
    let b = (bdy as f64) / (bdx as f64);
    let c = (ay as f64) - ((ax as f64) * a);
    let d = (by as f64) - ((bx as f64) * b);

    let ix = (d - c) / (a - b);
    let iy = a * ix + c;

    println!(
        "
        Hailstone A: {ax}, {ay}, {az} @ {adx}, {ady}, {adz} (y = {a}x + {c})
        Hailstone B: {bx}, {by}, {bz} @ {bdx}, {bdy}, {bdz} (y = {b}x + {d})
        Hailstones' paths will cross at x={ix:.3}, y={iy:.3}.
        "
    );

    (ix, iy)
}

fn do_it(input: &str, min: usize, max: usize) -> usize {
    let particles = parse(input);
    for (&a, &b) in particles.iter().tuple_combinations() {
        let intersection = find_intersection(a, b);
    }

    0
}

fn do_it_again(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn parse(input: &str) -> Vec<((usize, usize, usize), (isize, isize, isize))> {
    input
        .lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" @ ").unwrap();
            let poss: Vec<_> = pos.split(", ").map(|p| p.parse().unwrap()).collect();
            let vels: Vec<_> = vel.split(", ").map(|p| p.trim().parse().unwrap()).collect();
            ((poss[0], poss[1], poss[2]), (vels[0], vels[1], vels[2]))
        })
        .collect()
}

#[test]
fn test() {
    let input = r"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str(), 7, 27), 2);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

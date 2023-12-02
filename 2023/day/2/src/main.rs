use std::cmp::max;

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .filter(|(_, inners)| {
            inners
                .into_iter()
                .all(|&(red, green, blue)| red <= 12 && green <= 13 && blue <= 14)
        })
        .map(|(id, _)| id)
        .sum()
}

fn do_it_again(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|(_, inners)| {
            inners
                .into_iter()
                .reduce(|(r1, g1, b1), (r2, g2, b2)| (max(r1, r2), max(g1, g2), max(b1, b2)))
                .unwrap()
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

fn parse(line: &str) -> (usize, Vec<(usize, usize, usize)>) {
    let (game, inner) = line.split_once(": ").unwrap();

    (
        game[5..].parse().unwrap(),
        inner.split("; ").map(parse_inner).collect(),
    )
}

fn parse_inner(part: &str) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for thing in part.split(", ") {
        let (count_s, color) = thing.split_once(" ").unwrap();
        let count = count_s.parse().unwrap();
        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => panic!("unknown color"),
        }
    }

    (red, green, blue)
}

#[test]
fn test() {
    let input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 8);
    assert_eq!(do_it_again(input.as_str()), 2286)
}

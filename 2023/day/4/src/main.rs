use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> u32 {
    input
        .lines()
        .map(parse)
        .map(|(winning, mine)| winning.intersection(&mine).collect::<HashSet<_>>().len() as u32)
        .filter(|&w| w > 0)
        .map(|w| 2_u32.pow(w - 1))
        .sum()
}

fn do_it_again(input: &str) -> u32 {
    let wins: Vec<_> = input
        .lines()
        .map(parse)
        .map(|(winning, mine)| winning.intersection(&mine).collect::<HashSet<_>>().len() as u32)
        .collect();

    let mut copies = vec![1; wins.len()];

    for (i, n) in wins.iter().enumerate() {
        for _ in 0..copies[i] {
            for copy in 1..(n + 1) {
                copies[i + copy as usize] += 1;
            }
        }
    }

    copies.iter().sum()
}

fn parse(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (_, card) = line.split_once(": ").unwrap();
    let (winning, mine) = card.split_once(" | ").unwrap();
    (
        parse_inner(winning),
        parse_inner(mine),
    )
}

fn parse_inner(numbers: &str) -> HashSet<u32> {
    numbers
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| s != &"")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[test]
fn test() {
    let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 13);
    assert_eq!(do_it_again(input.as_str()), 30)
}

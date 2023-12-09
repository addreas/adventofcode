use std::fmt::{write, Display, Debug};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn count(hand: &Vec<u32>) -> (usize, usize) {
    let mut most = 1;
    let mut second_most = 1;
    for i in 2..15 {
        let count = hand.iter().filter(|c| **c == i).count();
        if count > most {
            second_most = most;
            most = count;
        } else if count > second_most {
            second_most = count;
        }
    }
    (most, second_most)
}

fn do_it(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(parse).collect();

    hands.sort_unstable_by_key(|(h, _)| (count(h), h.clone()));

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * *bet as usize)
        .sum()
}

fn count_again(hand: &Vec<u32>) -> (usize, usize) {
    let mut most = 0;
    let mut second_most = 0;
    let mut jokers = 0;
    for i in 2..15 {
        let count = hand.iter().filter(|c| **c == i).count();
        if i == 11 {
            jokers = count;
        } else if count > most {
            second_most = most;
            most = count;
        } else if count > second_most {
            second_most = count;
        }
    }

    (most + jokers, second_most)
}

fn do_it_again(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(parse).collect();

    hands.sort_unstable_by_key(|(h, _)| {
        (
            count_again(h),
            h.iter()
                .map(|i| match i {
                    11 => 0,
                    i => *i,
                })
                .collect::<Vec<_>>(),
        )
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * *bet as usize)
        .sum()
}


fn fmt(hand: &Vec<u32>) -> String {
    hand.iter()
        .map(|i| std::char::from_digit(*i, 16).unwrap())
        .collect::<String>()
        .to_uppercase()
        .replace("A", "T")
        .replace("B", "J")
        .replace("C", "Q")
        .replace("D", "K")
        .replace("E", "A")

}

fn parse(line: &str) -> (Vec<u32>, u32) {
    let (hand, bet) = line.split_once(" ").unwrap();

    (
        hand.replace("A", "E")
            .replace("K", "D")
            .replace("Q", "C")
            .replace("J", "B")
            .replace("T", "A")
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .collect(),
        bet.parse().unwrap(),
    )
}

#[test]
fn test() {
    let input = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 6440);
    assert_eq!(do_it_again(input.as_str()), 5905)
}

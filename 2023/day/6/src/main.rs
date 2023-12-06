fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input));
    println!("{}", do_it_again(input));
}

fn do_it(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|(time, distance)| {
            (1..(time - 1))
                .map(|hold_time| hold_time * (time - hold_time))
                .filter(|d| d > distance)
                .count()
        })
        .product()
}

fn do_it_again(input: &str) -> usize {
    let (time, distance) = parse_again(input);

    (1..(time - 1))
        .map(|hold_time| hold_time * (time - hold_time))
        .filter(|d| d > &distance)
        .count()
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let (times, distances) = input.split_once("\n").unwrap();

    times
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .zip(
            distances
                .split_once(":")
                .unwrap()
                .1
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap()),
        )
        .collect()
}

fn parse_again(input: &str) -> (u64, u64) {
    let (time, distance) = input.split_once("\n").unwrap();

    (
        time.replace(" ", "")
            .split_once(":")
            .unwrap()
            .1
            .parse()
            .unwrap(),
        distance
            .replace(" ", "")
            .trim()
            .split_once(":")
            .unwrap()
            .1
            .parse()
            .unwrap(),
    )
}

#[test]
fn test() {
    let input = "Time:      7  15   30\nDistance:  9  40  200";
    assert_eq!(do_it(input), 288);
    assert_eq!(do_it_again(input), 71503)
}

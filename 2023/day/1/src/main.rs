use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|l| l[0] * 10 + l[l.len() - 1])
        .sum()
}

fn do_it_again(input: &str) -> u32 {
    do_it(
        input
            .replace("oneight", "1")
            .replace("eight", "8")
            .replace("two", "2")
            .replace("one", "1")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("nine", "9")
            .replace("seven", "7")
            .as_str(),
    )
}

#[test]
fn test() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(do_it(input), 142);

    let input2 = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    assert_eq!(do_it_again(input2), 281)
}

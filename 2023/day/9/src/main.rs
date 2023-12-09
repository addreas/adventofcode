fn main() {
    let input = include_str!("../input").trim();
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn get_seqs(nums: Vec<isize>) -> Vec<Vec<isize>> {
    let mut seqs = vec![nums];
    while !seqs.last().unwrap().iter().all(|&n| n == 0) {
        seqs.push(
            seqs.last()
                .unwrap()
                .windows(2)
                .map(|c| c[1] - c[0])
                .collect(),
        );
    }
    seqs
}

fn do_it(input: &str) -> isize {
    input
        .lines()
        .map(parse)
        .map(get_seqs)
        .map(|seqs| {
            seqs.iter()
                .rev()
                .map(|s| *s.last().unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .sum()
}

fn do_it_again(input: &str) -> isize {
    input
        .lines()
        .map(parse)
        .map(get_seqs)
        .map(|seqs| {
            seqs.iter()
                .rev()
                .map(|s| *s.first().unwrap())
                .reduce(|a, b| b - a)
                .unwrap()
        })
        .sum()
}

fn parse(line: &str) -> Vec<isize> {
    line.split(" ").map(|n| n.parse().unwrap()).collect()
}

#[test]
fn test() {
    let input = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 114);
    assert_eq!(do_it_again(input.as_str()), 2)
}

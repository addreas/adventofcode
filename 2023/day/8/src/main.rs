use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim();
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> usize {
    let (path, edges) = parse(input);

    let mut current = "AAA";
    for i in 0..100000 {
        let choice = path.chars().nth(i % path.len()).unwrap();
        println!("{i}: {current}, {choice}");
        let &(left, right) = edges.get(current).unwrap();
        match choice {
            'L' => current = left,
            'R' => current = right,
            _ => panic!(),
        }
        if current == "ZZZ" {
            return i + 1;
        }
    }

    panic!("ran out of i")
}

fn do_it_again(input: &str) -> usize {
    let (path, edges) = parse(input);

    0
}

fn parse(line: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (path, edges) = line.split_once("\n\n").unwrap();

    (
        path,
        edges
            .split("\n")
            .map(|l| {
                let (name, pair) = l.split_once(" = ").unwrap();
                let (left, right) = pair[1..pair.len() - 1].split_once(", ").unwrap();
                (name, (left, right))
            })
            .collect(),
    )
}

#[test]
fn test() {
    let input = "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 2);
    let input2 = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input2.as_str()), 6);
    assert_eq!(do_it_again(input.as_str()), 0)
}

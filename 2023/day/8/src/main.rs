use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim();
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

struct PathFollower<'a> {
    path: Vec<char>,
    i: usize,

    current: &'a str,
    edges: &'a HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Iterator for PathFollower<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let choice = self.path[self.i % self.path.len()];

        let &(left, right) = self.edges.get(self.current).unwrap();
        match choice {
            'L' => self.current = left,
            'R' => self.current = right,
            _ => panic!(),
        }

        self.i += 1;
        Some((self.i, self.current))
    }
}

fn do_it(input: &str) -> usize {
    let (path, edges) = parse(input);

    let path_follower = PathFollower {
        path: path.chars().collect(),
        i: 0,
        current: "AAA",
        edges: &edges,
    };

    for (i, current) in path_follower {
        if current == "ZZZ" {
            return i;
        }
    }

    unreachable!()
}

fn do_it_again(input: &str) -> usize {
    let (path, edges) = parse(input);

    let initial: Vec<_> = edges
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| *node)
        .collect();

    let ends: Vec<_> = initial
        .iter()
        .map(|current| {
            let mut path_follower = PathFollower {
                path: path.chars().collect(),
                i: 0,
                current,
                edges: &edges,
            };

            path_follower
                .find_map(|(i, e)| e.ends_with("Z").then(|| i))
                .unwrap()
        })
        .collect();

    ends.into_iter()
        .reduce(num::integer::lcm)
        .unwrap()
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

    let input3 = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it_again(input3.as_str()), 6)
}

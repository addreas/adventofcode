use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn exec<'a>(instruction: &'a Instruction, part: (usize, usize, usize, usize)) -> Option<&'a str> {
    let (x, m, a, s) = part;

    if let Some((t, op, val)) = instruction.predicate {
        let n = match t {
            'x' => x,
            'm' => m,
            'a' => a,
            's' => s,
            _ => unreachable!(),
        };

        let comparison = match op {
            '>' => n > val,
            '<' => n < val,
            _ => unreachable!(),
        };

        if comparison {
            Some(instruction.target)
        } else {
            None
        }
    } else {
        Some(instruction.target)
    }
}

fn execute<'a>(
    instructions: &'a HashMap<&'a str, Vec<Instruction<'a>>>,
    part: (usize, usize, usize, usize),
) -> (Vec<&'a str>, bool) {
    let mut path = vec![];
    let mut current = "in";

    while current != "A" && current != "R" {
        path.push(current);

        let instrs = instructions.get(current).unwrap();
        current = instrs.iter().find_map(|inst| exec(inst, part)).unwrap();
    }

    (path, current == "A")
}

fn do_it(input: &str) -> usize {
    let (instructions, parts) = parse(input);
    parts
        .iter()
        .filter(|p| execute(&instructions, **p).1)
        .map(|(x, m, a, s)| x + m + a + s)
        .sum()
}

fn do_it_again(input: &str) -> usize {
    let (instructions, _) = parse(input);

    0
}

struct Instruction<'a> {
    target: &'a str,
    predicate: Option<(char, char, usize)>,
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, Vec<Instruction>>,
    Vec<(usize, usize, usize, usize)>,
) {
    let (instructions, parts) = input.split_once("\n\n").unwrap();

    (parse_instructions(instructions), parse_parts(parts))
}

fn parse_instructions(input: &str) -> HashMap<&str, Vec<Instruction>> {
    input
        .lines()
        .map(|l| {
            let (name, instructions) = l.split_once("{").unwrap();

            (
                name,
                instructions[..instructions.len() - 1]
                    .split(",")
                    .map(|instr| match instr.split_once(":") {
                        Some((pred, target)) => Instruction {
                            target,
                            predicate: Some((
                                pred.chars().nth(0).unwrap(),
                                pred.chars().nth(1).unwrap(),
                                pred[2..].parse().unwrap(),
                            )),
                        },
                        None => Instruction {
                            target: instr,
                            predicate: None,
                        },
                    })
                    .collect(),
            )
        })
        .collect()
}

fn parse_parts(input: &str) -> Vec<(usize, usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;

            l[1..l.len() - 1].split(",").for_each(|n| {
                let (k, v) = n.split_once("=").unwrap();
                let val = v.parse().unwrap();
                match k {
                    "x" => x = val,
                    "m" => m = val,
                    "a" => a = val,
                    "s" => s = val,
                    _ => unreachable!(),
                }
            });

            (x, m, a, s)
        })
        .collect()
}

#[test]
fn test() {
    let input = r"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 19114);

    assert_eq!(do_it_again(input.as_str()), 167409079868000);
}

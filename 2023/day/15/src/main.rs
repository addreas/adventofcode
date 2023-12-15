fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn hash(str: &str) -> usize {
    str.bytes().fold(0, |acc, b| (acc + b as usize) * 17 % 256)
}

fn do_it(input: &str) -> usize {
    parse(input).into_iter().map(hash).sum()
}

fn do_it_again(input: &str) -> usize {
    let instructions = parse(input).into_iter().map(parse_step);
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::with_capacity(256);
    boxes.resize(256, vec![]);

    for (key, op) in instructions {
        let index = hash(key);
        match op {
            Op::Remove => {
                boxes[index] = boxes[index]
                    .iter()
                    .cloned()
                    .filter(|(k, _)| *k != key)
                    .collect()
            }
            Op::Insert(value) => {
                if boxes[index].iter().find(|(k, _)| *k == key).is_some() {
                    boxes[index] = boxes[index]
                        .iter()
                        .cloned()
                        .map(|(k, v)| if k == key { (key, value) } else { (k, v) })
                        .collect()
                } else {
                    boxes[index].push((key, value))
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| b.len() > 0)
        .map(|(box_number, b)| {
            b.iter()
                .enumerate()
                .map(|(slot_number, (_, focal_length))| {
                    (box_number + 1) * (slot_number + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

fn parse(line: &str) -> Vec<&str> {
    line.trim().split(",").collect()
}

enum Op {
    Insert(usize),
    Remove,
}

fn parse_step(step: &str) -> (&str, Op) {
    if step.ends_with("-") {
        (&step[..step.len() - 1], Op::Remove)
    } else {
        let (key, value) = step.split_once("=").unwrap();
        (key, Op::Insert(value.parse().unwrap()))
    }
}

#[test]
fn test() {
    let input = "
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 1320);

    assert_eq!(do_it_again(input.as_str()), 145);
}

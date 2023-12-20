use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> usize {
    let modules = parse(input);
    let mut flip_flops: HashMap<&str, bool> = HashMap::new();
    let mut conjuncts: HashMap<&str, HashMap<&str, bool>> = HashMap::new();

    for (source, (_, targets)) in modules.iter() {
        for t in targets {
            if modules.get(t).unwrap().0 == "&" {
                // println!("{kind}{source} -> {t}");
                conjuncts
                    .entry(t)
                    .and_modify(|inputs| {
                        inputs.insert(source, false);
                    })
                    .or_insert(HashMap::from([(*source, false)]));
            }
        }
    }

    let mut high = 0;
    let mut low = 0;

    let mut signals = VecDeque::from([("button", false, "broadcaster")]);
    let mut visited = HashSet::new();

    while let Some((source, signal, target)) = signals.pop_front() {
        println!("{source} -{signal}-> {target}. {signals:?}");
        if signal {
            high += 1;
        } else {
            low += 1;
        }

        if visited.contains(&(source, signal, target)) {
            continue;
        }
        visited.insert((source, signal, target));

        if let Some((kind, nexts)) = modules.get(target) {
            if target == "broadcaster" {
                signals.extend(nexts.iter().map(|n| (target, signal, *n)));
            } else if *kind == "%" {
                let new_state = !flip_flops.get(target).unwrap_or(&false);
                flip_flops.insert(target, new_state);

                signals.extend(nexts.iter().map(|n| (target, new_state, *n)));
            } else if *kind == "&" {
                let inputs = conjuncts.get_mut(target).unwrap();
                inputs.insert(source, signal);

                let new_signal = !inputs.iter().all(|(_, s)| *s);

                signals.extend(nexts.iter().map(|n| (target, new_signal, *n)));
            };

        }
    }

    (1000 * high) * (1000 * low)
}

fn do_it_again(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn parse(input: &str) -> HashMap<&str, (&str, Vec<&str>)> {
    input
        .lines()
        .map(|l| {
            let (key, value) = l.split_once(" -> ").unwrap();

            let (kind, name) = if key == "broadcaster" {
                ("", key)
            } else {
                key.split_at(1)
            };

            (name, (kind, value.split(", ").collect()))
        })
        .collect()
}

#[test]
fn test() {
    let input = r"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 32000000);

    let input2 = r"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input2.as_str()), 11687500);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

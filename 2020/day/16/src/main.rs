#![feature(str_split_once)]
#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (fields, ticket, tickets) = parse(&input);
    println!("{}", scan_tickets(&fields, &tickets));
    let order = get_order(&fields, &tickets).unwrap();
    let res = ticket
        .iter()
        .zip(order.iter())
        .filter(|(_, f)| f.starts_with("departure"))
        .fold(1, |acc, (v, _)| acc * v);
    println!("{:?}", res);
}

fn scan_tickets(fields: &Vec<(&str, Vec<(usize, usize)>)>, tickets: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for ticket in tickets.iter() {
        for value in ticket.iter() {
            let any = fields.iter().any(|(_, ranges)| {
                ranges
                    .iter()
                    .any(|(low, high)| value >= low && value <= high)
            });
            if !any {
                res += value;
            }
        }
    }
    return res;
}

fn get_order<'a>(
    fields: &'a Vec<(&str, Vec<(usize, usize)>)>,
    tickets: &Vec<Vec<usize>>,
) -> Option<Vec<&'a str>> {
    let valid = tickets
        .iter()
        .filter_map(|ticket| {
            let sets = ticket
                .iter()
                .map(|value| {
                    fields
                        .iter()
                        .filter(|(_, ranges)| {
                            ranges
                                .iter()
                                .any(|(low, high)| value >= low && value <= high)
                        })
                        .map(|(field, _)| *field)
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();
            if sets.iter().any(|s| s.is_empty()) {
                return None;
            }
            return Some(sets);
        })
        .collect::<Vec<_>>();
    let options = fields
        .iter()
        .enumerate()
        .map(|(i, _)| {
            valid
                .iter()
                .map(|v| v.get(i).unwrap().to_owned())
                .fold_first(|acc, item| acc.intersection(&item).map(|x| *x).collect())
                .unwrap()
        })
        .collect::<Vec<_>>();

    return pick(options);
}

fn pick(options: Vec<HashSet<&str>>) -> Option<Vec<&str>> {
    if let Some((head, tail)) = options.split_first() {
        for option in head.iter() {
            let rest = tail
                .iter()
                .map(|s| {
                    let mut new = s.clone();
                    new.remove(option);
                    return new;
                })
                .collect();
            if let Some(mut x) = pick(rest) {
                let mut vec = vec![*option];
                vec.append(x.as_mut());
                return Some(vec);
            }
        }
        return None;
    } else {
        return Some([].to_vec());
    }
}

#[test]
fn test_example() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"
    .to_string();

    let (fields, _ticket, tickets) = parse(&input);
    assert_eq!(scan_tickets(&fields, &tickets), 71);
}

#[test]
fn test_example2() {
    let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"
    .to_string();

    let (fields, _ticket, tickets) = parse(&input);
    assert_eq!(
        get_order(&fields, &tickets),
        Some(vec!["row", "class", "seat"])
    );
}

fn parse<'a>(
    input: &'a String,
) -> (
    Vec<(&'a str, Vec<(usize, usize)>)>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let mut sections = input.split("\n\n");
    let fields = sections
        .next()
        .unwrap()
        .split("\n")
        .map(|r| {
            let (name, rest) = r.split_once(": ").unwrap();
            let ranges = rest
                .split(" or ")
                .map(|r| {
                    r.split_once("-")
                        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .collect::<Vec<_>>();

            return (name, ranges);
        })
        .collect::<Vec<_>>();
    let ticket = sections
        .next()
        .unwrap()
        .strip_prefix("your ticket:\n")
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let tickets = sections
        .next()
        .unwrap()
        .strip_prefix("nearby tickets:\n")
        .unwrap()
        .split("\n")
        .filter(|t| *t != "")
        .map(|t| {
            t.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    return (fields, ticket, tickets);
}

#![feature(str_split_once)]
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (fields, tickets, others) = parse(&input);
    println!("{}", scan_others(&fields, &others));
    println!("{:?}", get_mapping(&fields, &tickets, &others));
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
    let others = sections
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

    return (fields, ticket, others);
}

fn scan_others(fields: &Vec<(&str, Vec<(usize, usize)>)>, others: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for ticket in others.iter() {
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

fn get_mapping<'a>(
    fields: &'a Vec<(&str, Vec<(usize, usize)>)>,
    ticket: &Vec<usize>,
    others: &Vec<Vec<usize>>,
) -> Vec<&'a str> {
    let candidates = [("row", 0), ("seat", 0)];
    let valid = others
        .iter()
        .filter(|other| {
            other.iter().all(|value| {
                fields.iter().any(|(_, ranges)| {
                    ranges
                        .iter()
                        .any(|(low, high)| value >= low && value <= high)
                })
            })
        })
        .collect::<Vec<_>>();

    println!("{:?}", valid);
    println!("{:?}", ticket);
    return vec![""];
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

    let (fields, _tickets, others) = parse(&input);
    assert_eq!(scan_others(&fields, &others), 71);
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

    let (fields, tickets, others) = parse(&input);
    println!("{:?}", get_mapping(&fields, &tickets, &others));
}

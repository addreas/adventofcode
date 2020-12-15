use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Chunk {
    mask: String,
    assignments: Vec<(usize, usize)>,
}

fn read_assignments() -> Vec<Chunk> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    return input
        .split("mask = ")
        .map(|chunk| chunk.split("\n"))
        .map(|mut chunkparts| Chunk {
            mask: chunkparts.next().unwrap().to_string(),
            assignments: chunkparts
                .filter(|assignment| *assignment != "")
                .map(|assignment| assignment.split(" = "))
                .map(|mut assignment| {
                    (
                        assignment
                            .next()
                            .unwrap()
                            .strip_prefix("mem[")
                            .unwrap()
                            .strip_suffix("]")
                            .unwrap()
                            .parse()
                            .unwrap(),
                        assignment.next().unwrap().parse().unwrap(),
                    )
                })
                .collect(),
        })
        .collect();
}

fn run_one(chunks: &Vec<Chunk>) -> usize {
    let mut mem = HashMap::<usize, usize>::new();
    for chunk in chunks.iter() {
        let char_enum = &chunk.mask.chars().rev().enumerate().collect::<Vec<_>>();

        let one_mask = char_enum
            .iter()
            .filter(|(_, x)| *x == '1')
            .fold(0_usize, |acc, (i, _)| acc + (1 << i));

        let zero_mask = char_enum
            .iter()
            .filter(|(_, x)| *x == '0')
            .fold(!0_usize, |acc, (i, _)| acc - (1 << i));

        for (addr, val) in chunk.assignments.iter() {
            mem.insert(*addr, *val | one_mask & zero_mask);
        }
    }
    return mem.values().sum();
}

fn run_two(chunks: &Vec<Chunk>) -> usize {
    let mut mem = HashMap::<usize, usize>::new();
    for chunk in chunks.iter() {
        let char_enum = &chunk.mask.chars().rev().enumerate().collect::<Vec<_>>();

        let one_mask = char_enum
            .iter()
            .filter(|(_, x)| *x == '1')
            .fold(0, |acc, (i, _)| acc + (1 << i));

        let floating = char_enum
            .iter()
            .filter(|(_, x)| *x == 'X')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for (mut addr, val) in chunk.assignments.iter() {
            addr |= one_mask;
            let addrs = get_addrs(addr, floating.as_slice());
            for actual_addr in addrs {
                mem.insert(actual_addr, *val);
            }
        }
    }
    return mem.values().sum();
}

fn get_addrs(addr: usize, floating: &[&usize]) -> Vec<usize> {
    match floating.split_first() {
        None => vec![addr],
        Some((&bit_index, rest)) => [
            get_addrs(addr | (1 << bit_index), rest),
            get_addrs(addr & !(1 << bit_index), rest),
        ]
        .concat(),
    }
}

fn main() {
    let chunks = read_assignments();
    println!("{:?}", run_one(&chunks));
    println!("{:?}", run_two(&chunks));
}

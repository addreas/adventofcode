use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let starting = input
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>();

    println!("{}", run(starting.as_slice(), 2020));
    println!("{}", run(starting.as_slice(), 30000000));
}

fn run(starting: &[usize], target: usize) -> usize {
    let mut hashy: HashMap<usize, usize> = HashMap::with_capacity(target);
    for (i, val) in starting.iter().take(starting.len() - 1).enumerate() {
        hashy.insert(*val, i);
    }
    let mut prev: usize = *starting.last().expect("non empty starting list");
    let mut curr: usize = 0;
    for i in starting.len() - 1..target - 1 {
        curr = match hashy.get(&prev) {
            None => 0,
            Some(x) => i - *x,
        };
        hashy.insert(prev, i);
        prev = curr;
    }

    return curr;
}

#[test]
fn test_small() {
    assert_eq!(run(&[0, 3, 6], 10), 0);
}

#[test]
fn test_examples() {
    assert_eq!(run(&[1, 3, 2], 2020), 1);
    assert_eq!(run(&[2, 1, 3], 2020), 10);
    assert_eq!(run(&[1, 2, 3], 2020), 27);
    assert_eq!(run(&[2, 3, 1], 2020), 78);
    assert_eq!(run(&[3, 2, 1], 2020), 438);
    assert_eq!(run(&[3, 1, 2], 2020), 1836);
}

#[test]
#[ignore]
fn test_big() {
    assert_eq!(run(&[0, 3, 6], 30000000), 175594);
    assert_eq!(run(&[1, 3, 2], 30000000), 2578);
    assert_eq!(run(&[2, 1, 3], 30000000), 3544142);
    assert_eq!(run(&[1, 2, 3], 30000000), 261214);
    assert_eq!(run(&[2, 3, 1], 30000000), 6895259);
    assert_eq!(run(&[3, 2, 1], 30000000), 18);
    assert_eq!(run(&[3, 1, 2], 30000000), 362);
}

#![feature(iter_array_chunks)]
#![feature(let_chains)]
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::fs;


fn main() {
    let res: i64 = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l))
        .array_chunks()
        .map(|[a,b,_]| (a.unwrap(), b.unwrap()))
        .map(|(a, b)| compare(&a, &b, ""))
        .zip(1..)
        .filter(|(cmp, _)| cmp.is_le())
        .map(|(_, index)| index)
        .sum();
    println!("sum: {res}");

    let d1 = json!([[2]]);
    let d2 = json!([[6]]);
    let mut divider = vec!(d1.clone(), d2.clone());
    let mut res2: Vec<Value> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .filter(|l| *l != "")
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    res2.append(&mut divider);
    res2.sort_by(|l, r| compare(l, r, ""));
    if let Some((_, d1_index)) = res2.iter().zip(1..).find(|(l, _)| **l == d1) &&
        let Some((_, d2_index)) = res2.iter().zip(1..).find(|(l, _)| **l == d2) {
            println!("decoder key: {}", d1_index * d2_index)
    }

}

fn compare(left: &Value, right: &Value, indent: &str) -> Ordering {
    println!("{}- Compare {:?} vs {:?}", indent, left, right);
    let next_indent = indent.to_string() + "  ";
    let res = match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            let res = l.as_i64().cmp(&r.as_i64());
            if res == Ordering::Less {
                println!("{}- Left side is smaller, so inputs are *in the right order*", next_indent);
            } else if res == Ordering::Greater {
                println!("{}- Right side is smaller, so inputs are *not* in the right order", next_indent);
            }

            res
        },
        (left, Value::Number(r)) => {
            println!("{}- Mixed types; convert right to [{}] and retry comparison", next_indent, r);
            compare(left, &json!([r]), next_indent.as_str())
        },
        (Value::Number(l), right) => {
            println!("{}- Mixed types; convert left to [{}] and retry comparison", next_indent, l);
            compare(&json!([l]), right, next_indent.as_str())
        },
        (Value::Array(left), Value::Array(right)) => {
            for (l, r) in std::iter::zip(left.iter(), right.iter()) {
                let res = compare(l, r, next_indent.as_str());
                if res == Ordering::Equal {
                    continue;
                } else {
                    return res;
                }
            }
            let res = left.len().cmp(&right.len());
            if res == Ordering::Less {
                println!("{}- Left side ran out of items, so inputs are *in the right order*", next_indent);
            } else if res == Ordering::Greater {
                println!("{}- Right side ran out of items, so inputs are *not* in the right order", next_indent);
            }
            return res;
        },
        _ => Ordering::Equal
    };

    // println!("{}- Compare {:?} vs {:?} = {:?}", indent, left, right, res);
    return res;
}

#[test]
fn test_small() {
    assert_eq!(compare(&json!([]), &json!([]), ""), Ordering::Less);
}

#[test]
fn test_examples() {
    println!("\n== Pair 1 ==");
    assert_eq!(compare(&json!([1,1,3,1,1]), &json!([1,1,5,1,1]), ""), Ordering::Less);
    println!("\n== Pair 2 ==");
    assert_eq!(compare(&json!([[1],[2,3,4]]), &json!([[1],4]), ""), Ordering::Less);
    println!("\n== Pair 3 ==");
    assert_eq!(compare(&json!([9]), &json!([[8,7,6]]), ""), Ordering::Greater);
    println!("\n== Pair 4 ==");
    assert_eq!(compare(&json!([[4,4],4,4]), &json!([[4,4],4,4,4]), ""), Ordering::Less);
    println!("\n== Pair 5 ==");
    assert_eq!(compare(&json!([7,7,7,7]), &json!([7,7,7]), ""), Ordering::Greater);
    println!("\n== Pair 6 ==");
    assert_eq!(compare(&json!([]), &json!([3]), ""), Ordering::Less);
    println!("\n== Pair 7 ==");
    assert_eq!(compare(&json!([[[]]]), &json!([[]]), ""), Ordering::Greater);
    println!("\n== Pair 8 ==");
    assert_eq!(compare(&json!([1,[2,[3,[4,[5,6,7]]]],8,9]), &json!([1,[2,[3,[4,[5,6,0]]]],8,9]), ""), Ordering::Greater);
}

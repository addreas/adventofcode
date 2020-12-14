use std::io;

fn read_earliest() -> i64 {
    let mut earliest = String::new();
    io::stdin().read_line(&mut earliest).unwrap();

    return i64::from_str_radix(earliest.trim().as_ref(), 10).unwrap();
}

fn read_busses() -> Vec<Option<i64>> {
    let mut busses = String::new();
    io::stdin().read_line(&mut busses).unwrap();

    return busses
        .split(",")
        .map(|s| i64::from_str_radix(s.trim().as_ref(), 10).ok())
        .collect();
}

fn main() {
    let earliest = read_earliest();
    let busses = read_busses();

    let res = busses
        .iter()
        .filter_map(|x| x.as_ref())
        .map(|x| (x, (earliest / x + 1) * x - earliest))
        .min_by_key(|(_, delta)| *delta);

    println!("{:?}", res.map(|(id, delta)| *id * delta).unwrap());

    let (residues, modulii): (Vec<_>, Vec<_>) = busses
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != None)
        .map(|(i, &x)| (i as i64, x.unwrap()))
        .unzip();

    println!("{:?}", modulii.iter().product::<i64>() - chinese_remainder(&residues, &modulii).unwrap());
}

 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
 
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

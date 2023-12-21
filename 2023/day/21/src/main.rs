use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref(), 64));
    println!("{}", do_it_again(input.as_ref(), 26501365));
}

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
   let mut  res = vec![
       (x, y+1),
       (x+1, y),
       ];
    if y > 0 {
        res.push((x, y-1));

    }
    if x > 0 {
        res.push((x-1, y));
    }

    res
}


fn do_it(input: &str, steps: usize) -> usize {
    let (garden, start, _) = parse(input);

    let mut visited = HashSet::from([start]);
    for i in 1..(steps+1) {
        let mut visiting = HashSet::new();

        for &(x, y) in visited.iter() {
            for &(nx, ny) in neighbors(x, y).iter() {
                if garden.contains(&(nx, ny)) {
                    visiting.insert((nx, ny));
                }
            }
        }
        visited = visiting;
        // print(&garden, &visited);
    }

    visited.len()
}

fn neighbors_again(x: isize, y: isize) -> Vec<(isize, isize)> {
    vec![
        (x, y - 1),
        (x, y + 1),
        (x - 1, y),
        (x + 1, y),
    ]
}

fn do_it_again(input: &str, steps: usize) -> usize {
    let (garden, start, (xmax, ymax)) = parse(input);

    let mut visited = HashSet::from([(start.0 as isize, start.1 as isize)]);
    for i in 1..(steps+1) {
        let mut visiting = HashSet::new();
        for &(x, y) in visited.iter() {
            for &(nx, ny) in neighbors_again(x, y).iter() {
                if garden.contains(&(nx.rem_euclid(xmax as isize) as usize, ny.rem_euclid(ymax as isize) as usize)) {
                    visiting.insert((nx, ny));
                }
            }
        }
        visited = visiting;
        println!("step: {i}/{steps} {}/{} == {}", i*i, visited.len(), ((i*i) as f64)/(visited.len() as f64));
    }

    visited.len()
}

fn print(garden: &HashSet<(usize, usize)>, visited: &HashSet<(usize,usize)>) {
    let xmax = *garden.iter().map(|(x, _)| x).max().unwrap();
    let ymax = *garden.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..(ymax+1) {
        for x in 0..(xmax+1) {
            print!("{}",
                if visited.contains(&(x, y)) {
                    "O"
                } else if garden.contains(&(x, y)) {
                    "."
                } else {
                    "#"
                }
            )
        }
        println!()
    }

    let stepxmin = *visited.iter().map(|(x, _)| x).min().unwrap();
    let stepymin = *visited.iter().map(|(_, y)| y).min().unwrap();
    let stepxmax = *visited.iter().map(|(x, _)| x).max().unwrap();
    let stepymax = *visited.iter().map(|(_, y)| y).max().unwrap();

    println!("x: {stepxmin}..{stepxmax}, y: {stepymin}..{stepymax}, dx: {}, dy: {}", stepxmax - stepxmin, stepymax - stepymin);
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut garden = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
            }

            if c != '#' {
                garden.insert((x, y));
            }
        }
    }

    (garden, start, (input.lines().next().unwrap().len(), input.lines().count()))
}

#[test]
fn test() {
    let input = r"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str(), 6), 16);

    assert_eq!(do_it_again(input.as_str(), 6), 16);
    assert_eq!(do_it_again(input.as_str(), 10), 50);
    assert_eq!(do_it_again(input.as_str(), 50), 1594);
    assert_eq!(do_it_again(input.as_str(), 100), 6536);
    assert_eq!(do_it_again(input.as_str(), 500), 167004);
    // assert_eq!(do_it_again(input.as_str(), 1000), 668697);
    // assert_eq!(do_it_again(input.as_str(), 5000), 16733044);
}

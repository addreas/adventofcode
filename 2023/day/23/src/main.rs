use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> usize {
    let (nodes, start, end) = parse(input);

    let mut q = VecDeque::from([vec![start]]);
    let mut finished= vec![];
    while let Some(path) = q.pop_front() {
        for edge in nodes.get(path.last().unwrap()).unwrap() {
            if edge == &end {
                finished.push(path.clone());
            }

            if !path.contains(edge) {
                let mut new_path = path.clone();
                new_path.push(*edge);
                q.push_back(new_path);
            }
        }
    }

    // for f in finished.iter() {
    //     println!("{}: {f:?}", f.len());
    // }

    finished.iter().map(|p| p.len()).max().unwrap()
}

fn do_it_again(input: &str) -> usize {
    let (nodes, start, end) = parse(input);

    0
}

fn parse(
    input: &str,
) -> (
    HashMap<(usize, usize), Vec<(usize, usize)>>,
    (usize, usize),
    (usize, usize),
) {
    let all: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' if y > 0=> Some(((x, y), vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)])),
                '.' => Some(((x, y), vec![(x - 1, y), (x + 1, y), (x, y + 1)])),
                '>' => Some(((x, y), vec![(x + 1, y)])),
                '<' => Some(((x, y), vec![(x - 1, y)])),
                '^' => Some(((x, y), vec![(x, y - 1)])),
                'v' => Some(((x, y), vec![(x, y + 1)])),
                _ => None,
            })
        })
        .collect();

    let exists: HashSet<_> = all.iter().map(|&(p, _)| p).collect();

    let mut nodes = HashMap::new();
    for ((x, y), edges) in all.iter() {
        nodes.insert(
            (*x, *y),
            edges
                .clone()
                .into_iter()
                .filter(|c| exists.contains(c))
                .collect(),
        );
    }

    (nodes, all.first().unwrap().0, all.last().unwrap().0)
}
#[test]
fn test() {
    let input = r"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 94);

    //assert_eq!(do_it_again(input.as_str()), 0);
}

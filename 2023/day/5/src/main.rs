fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn follow_map(from: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    map.iter()
        .find_map(|&(dest, source, len)| {
            if from >= source && from <= source + len {
                Some(from + (dest - source))
            } else {
                None
            }
        })
        .unwrap_or(from)
}

fn do_it(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |from, map| follow_map(from, map)))
        .min()
        .unwrap()
}

fn do_it_again(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    seeds
        .chunks_exact(2)
        .flat_map(|range| range[0]..(range[0] + range[1]))
        .map(|seed| maps.iter().fold(seed, |from, map| follow_map(from, map)))
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();

    (
        seeds
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect(),
        rest.split("\n\n").map(parse_inner).collect(),
    )
}

fn parse_inner(map: &str) -> Vec<(i64, i64, i64)> {
    let (_, rest) = map.split_once(" map:\n").unwrap();

    rest.lines()
        .map(|l| {
            l.split(" ")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|x| (x[0], x[1], x[2]))
        .collect()
}

#[test]
fn test() {
    let input = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "
    .trim()
    .replace("        ", "");

    let (seeds, maps) = parse(&input);

    assert_eq!(follow_map(79, &maps[0]), 81);
    assert_eq!(follow_map(14, &maps[0]), 14);
    assert_eq!(follow_map(55, &maps[0]), 57);
    assert_eq!(follow_map(13, &maps[0]), 13);

    assert_eq!(do_it(input.as_str()), 35);
    assert_eq!(do_it_again(input.as_str()), 46)
}

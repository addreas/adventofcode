#![feature(iter_array_chunks)]
#![feature(let_chains)]
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::ops::Range;

type P = (i64, i64);

fn main() {
    let pairs = get_pairs(fs::read_to_string("input.txt").unwrap());
    let empty = empty_on_line(&pairs, 2_000_000);
    println!(
        "empty: {}",
        empty.into_iter().map(|r| r.count()).sum::<usize>()
    );
}

fn hatty_dist((x1, y1): P, (x2, y2): P) -> i64 {
    (x1.abs_diff(x2) + y1.abs_diff(y2)).try_into().unwrap()
}

fn xy_ranges((x, y): P, (dx, dy): P) -> (Range<i64>, Range<i64>) {
    (
        if x - dx < x + dx {
            x - dx..x + dx
        } else {
            x + dx..x - dx
        },
        if y - dy < y + dy {
            y - dy..y + dy
        } else {
            y + dy..y - dy
        },
    )
}

fn empty_on_line(pairs: &Vec<(P, P)>, target_y: i64) -> Vec<Range<i64>> {
    let mut empty: Vec<Range<i64>> = Vec::new();
    let (sensors, beacons) = split_pairs(pairs);
    let sbs: BTreeSet<_> = sensors
        .union(&beacons)
        .filter(|(_, y)| *y == target_y)
        .collect();
    println!("sbs: {:?}", sbs);

    for (sensor, beacon) in pairs {
        let distance = hatty_dist(*sensor, *beacon);
        let (rx, ry) = xy_ranges(*sensor, (distance, distance));

        if !ry.contains(&target_y) {
            continue;
        }

        let sx = sensor.0;

        let dy = hatty_dist(*sensor, (sx, target_y));
        let dx = distance - dy;

        let ranges = sbs
            .iter()
            .fold(vec![(sx - dx, sx + dx)], |ranges, (x, _)|
                ranges
                    .into_iter()
                    .flat_map(|(start, end)| {
                        if !(start..end).contains(&x) {
                            vec![(start, end)]
                        } else {
                            vec![(start, *x), (*x + 1, end)]
                        }
                    }).collect()
        );
        println!(
            "sensor: {sensor:?}, beacon: {beacon:?}, distance: {distance}, ranges: {:?}",
            &ranges
        );

        empty.extend(ranges.into_iter().map(|(start, end)| start..end));


        // print(pairs, &empty)
    }

    println!("empty: {empty:?}");

    return empty;
}

fn get_pairs(input: String) -> Vec<(P, P)> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    input
        .lines()
        .filter_map(|l| {
            re.captures(&l).and_then(|cap| {
                Some((
                    (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                    (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
                ))
            })
        })
        .collect()
}
fn split_pairs(pairs: &Vec<(P, P)>) -> (BTreeSet<P>, BTreeSet<P>) {
    return pairs.iter().cloned().unzip();
}

#[test]
fn test() {
    let pairs = get_pairs(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "
        .to_string(),
    );

    println!("{pairs:?}");

    assert!(pairs.contains(&((9, 16), (10, 16))));
    assert!(pairs.contains(&((12, 14), (10, 16))));
    assert!(pairs.contains(&((10, 20), (10, 16))));
    assert!(pairs.contains(&((14, 17), (10, 16))));

    let empty = empty_on_line(&pairs, 10);
    // print(&pairs, empty);

    assert_eq!(empty.into_iter().map(|r| r.count()).sum::<usize>(), 26);
}

fn print(pairs: &Vec<(P, P)>, empty: &BTreeSet<P>) {
    let (sensors, beacons) = split_pairs(pairs);
    let all: Vec<_> = empty
        .union(&sensors.union(&beacons).cloned().collect())
        .cloned()
        .collect();
    let (min_x, min_y, max_x, max_y) = (
        all.iter()
            .min_by_key(|(x, _)| x)
            .map(|(x, _)| x)
            .unwrap()
            .to_owned(),
        all.iter()
            .min_by_key(|(_, y)| y)
            .map(|(_, y)| y)
            .unwrap()
            .to_owned(),
        all.iter()
            .max_by_key(|(x, _)| x)
            .map(|(x, _)| x)
            .unwrap()
            .to_owned(),
        all.iter()
            .max_by_key(|(_, y)| y)
            .map(|(_, y)| y)
            .unwrap()
            .to_owned(),
    );

    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match &(x, y) {
                i if empty.contains(i) => "#",
                i if sensors.contains(i) => "S",
                i if beacons.contains(i) => "B",
                _ => ".",
            })
            .collect::<String>();

        println!("{y:3} {}", line)
    }
}

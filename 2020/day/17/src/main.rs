#![feature(type_alias_impl_trait)]
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

trait Point: Eq + Hash + Sized {
    type PointIterator: Iterator<Item = Self>;
    fn neighbors(&self) -> Self::PointIterator;
}

struct World<T: Point>(HashSet<T>);

impl<T: Point> World<T> {
    fn gen(self) -> World<T> {
        return self
            .0
            .iter()
            .map(|p| p.neighbors().collect::<HashSet<T>>())
            .flatten()
            .filter(|c| {
                let count = self.0.intersection(&c.neighbors().collect()).count();
                if self.0.contains(c) {
                    return 2 <= count && count <= 3;
                } else {
                    return count == 3;
                }
            })
            .collect();
    }
    fn len(&self) -> usize {
        return self.0.len();
    }
}

impl<T: Point> FromIterator<T> for World<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        return World(it.into_iter().collect());
    }
}

impl Point for (isize, isize, isize) {
    type PointIterator = impl Iterator<Item = Self>;

    fn neighbors(&self) -> Self::PointIterator {
        let neighbors = (-1..=1)
            .map(|i| {
                (-1..=1)
                    .map(move |j| (-1..=1).map(move |k| (i, j, k)))
                    .flatten()
            })
            .flatten()
            .filter(|p| *p != (0, 0, 0));
        let (p1, p2, p3) = *self;
        return neighbors.map(move |(o1, o2, o3)| (p1 + o1, p2 + o2, p3 + o3));
    }
}

impl Point for (isize, isize, isize, isize) {
    type PointIterator = impl Iterator<Item = Self>;

    fn neighbors(&self) -> Self::PointIterator {
        let neighbors = (-1..=1)
            .map(|i| {
                (-1..=1)
                    .map(move |j| {
                        (-1..=1)
                            .map(move |k| (-1..=1).map(move |l| (i, j, k, l)))
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
            .filter(|p| *p != (0, 0, 0, 0));
        let (p1, p2, p3, p4) = *self;
        return neighbors.map(move |(o1, o2, o3, o4)| (p1 + o1, p2 + o2, p3 + o3, p4 + o4));
    }
}

fn main() {
    let initial_plane: HashSet<(isize, isize)> = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x as isize, y as isize)),
                    '.' => None,
                    _ => panic!("unknown input"),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let initial3d: World<(isize, isize, isize)> = initial_plane
        .iter()
        .map(|(x, y)| (*x, *y, 0_isize))
        .collect();
    println!("{:?}", initial3d.gen().gen().gen().gen().gen().gen().len());

    let initial4d: World<(isize, isize, isize, isize)> =
        initial_plane.iter().map(|(x, y)| (*x, *y, 0, 0)).collect();
    println!("{:?}", initial4d.gen().gen().gen().gen().gen().gen().len());
}

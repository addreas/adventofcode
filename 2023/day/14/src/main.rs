fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> isize {
    let board = parse(input);

    let mut empty = vec![0; board[0].len()];

    let mut total_load = 0;

    for (y, line) in board.iter().enumerate() {
        for (x, thing) in line.iter().enumerate() {
            match thing {
                Some(Rock::Round) => {
                    let load = (board.len() as isize) - ((y as isize) - empty[x]);
                    total_load += load;
                    // println!("{x},{y}, {load}, {total_load}, {}, {}", board.len(), empty[x]);
                },
                Some(Rock::Cube) => empty[x] = 0,
                None => empty[x] += 1,
            }
        }
    }

    total_load
}



fn do_it_again(input: &str) -> isize {
    parse(input).iter().map(|_| 0).sum()
}

enum Rock {
    Round,
    Cube,
}

fn parse(input: &str) -> Vec<Vec<Option<Rock>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Cube),
                    _ => unreachable!()
                })
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    let input = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), 136);

    assert_eq!(do_it_again(input.as_str()), 64);
}

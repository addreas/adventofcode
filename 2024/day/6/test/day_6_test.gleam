import gleam/io
import gleam/list
import gleam/set
import gleam/string
import gleeunit
import gleeunit/should

import day_6.{
  Down, Left, Right, Up, do_it, do_it_again, highlight_print, parse, turn_left,
  walk,
}

pub fn main() {
  gleeunit.main()
}

const input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"

// pub fn parse_test() {
//   input
//   |> string.trim()
//   |> parse()
//   |> io.debug()
// }

// pub fn do_it_test() {
//   input
//   |> string.trim()
//   |> do_it()
//   |> should.equal(41)
// }

// pub fn do_it_again_test() {
//   input
//   |> string.trim()
//   |> do_it_again()
//   |> should.equal(6)
// }

fn print(in) {
  use item <- set.each(in)
  io.debug(item)
}

pub fn walk_test() {
  let #(start, world) = parse(input)

  let original = walk(start, Up, set.new(), world)
  io.debug(original)

  original
  |> set.map(fn(step) {
    let #(x, y, dir) = step
    let has_match = set.contains(original, #(x, y, turn_left(dir)))
    #(x, y, has_match)
  })
  |> print

  let crosses = [#(4, 6), #(6, 6), #(6, 7), #(2, 8), #(4, 8), #(7, 8)]
  let directions = [Up, Down, Left, Right]

  crosses
  |> list.map(fn(cross) {
    let #(x, y) = cross

    let match =
      directions
      |> list.any(fn(dir) {
        io.debug(set.contains(original, #(x, y, dir)))
        && io.debug(set.contains(original, #(x, y, turn_left(dir))))
      })

    #(cross, match)
  })
  |> set.from_list()
  |> print
  // io.debug(#(3, 6))
  // io.debug(#(set.contains(original, #(4, 6, Left)), #(4, 6, Left)))
  // io.debug(#(set.contains(original, #(4, 6, turn_left(Left))), turn_left(Left)))
  // io.debug(#(set.contains(original, #(4, 6, Up)), #(4, 6, Up)))
  // io.debug(#(6, 7))
  // io.debug(#(set.contains(original, #(6, 6, Down)), #(6, 6, Down)))
  // io.debug(#(set.contains(original, #(6, 6, turn_left(Down))), turn_left(Down)))
  // io.debug(#(set.contains(original, #(6, 6, Left)), #(6, 6, Left)))
  // io.debug(#(7, 7))
  // io.debug(#(set.contains(original, #(6, 7, Right)), #(6, 7, Right)))
  // io.debug(#(
  //   set.contains(original, #(6, 7, turn_left(Right))),
  //   turn_left(Right),
  // ))
  // io.debug(#(set.contains(original, #(6, 7, Down)), #(6, 7, Down)))
  // io.debug(#(1, 8))
  // io.debug(#(set.contains(original, #(2, 8, Left)), #(2, 8, Left)))
  // io.debug(#(set.contains(original, #(2, 8, turn_left(Left))), turn_left(Left)))
  // io.debug(#(set.contains(original, #(2, 8, Up)), #(2, 8, Up)))
  // io.debug(#(3, 8))
  // io.debug(#(set.contains(original, #(4, 8, Left)), #(4, 8, Left)))
  // io.debug(#(set.contains(original, #(4, 8, turn_left(Left))), turn_left(Left)))
  // io.debug(#(set.contains(original, #(4, 8, Up)), #(4, 8, Up)))
  // io.debug(#(7, 9))
  // io.debug(#(set.contains(original, #(7, 8, Down)), #(7, 8, Down)))
  // io.debug(#(set.contains(original, #(7, 8, turn_left(Down))), turn_left(Down)))
  // io.debug(#(set.contains(original, #(7, 8, Left)), #(7, 8, Left)))
}

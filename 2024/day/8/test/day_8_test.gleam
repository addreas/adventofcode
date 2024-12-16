import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_8.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"

pub fn parse_test() {
  input
  |> string.trim()
  |> parse()
  |> io.debug()
}

pub fn do_it_test() {
  input
  |> string.trim()
  |> do_it()
  |> should.equal(14)
}

pub fn do_it_again_test() {
  input
  |> string.trim()
  |> do_it_again()
  |> should.equal(14)
}

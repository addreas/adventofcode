import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_2.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
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
  |> should.equal(2)
}

pub fn do_it_again_test() {
  input
  |> string.trim()
  |> do_it_again()
  |> should.equal(4)
}

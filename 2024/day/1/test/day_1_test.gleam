import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_1.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
3   4
4   3
2   5
1   3
3   9
3   3
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
  |> should.equal(11)
}

pub fn do_it_again_test() {
  input
  |> string.trim()
  |> do_it_again()
  |> should.equal(31)
}

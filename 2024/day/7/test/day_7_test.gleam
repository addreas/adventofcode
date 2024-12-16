import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_7.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
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
  |> should.equal(3749)
}

pub fn do_it_again_test() {
  input
  |> string.trim()
  |> do_it_again()
  |> should.equal(11_387)
}

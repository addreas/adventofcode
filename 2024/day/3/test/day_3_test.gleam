import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_3.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
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
  |> should.equal(161)
}

const input2 = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"

pub fn do_it_again_test() {
  input2
  |> string.trim()
  |> do_it_again()
  |> should.equal(48)
}

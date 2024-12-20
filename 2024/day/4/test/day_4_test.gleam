import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_4.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
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
  |> should.equal(18)
}

const input2 = "
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"

pub fn do_it_again_test() {
  input2
  |> string.trim()
  |> do_it_again()
  |> should.equal(9)
}

import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile.{read}

pub fn main() {
  let assert Ok(input) = read("./input")
  io.debug(do_it(string.trim(input)))
  io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  let #(left, right) =
    parse(input)
    |> list.unzip()

  list.zip(list.sort(left, int.compare), list.sort(right, int.compare))
  |> list.map(fn(ids) {
    let #(a, b) = ids
    int.absolute_value(a - b)
  })
  |> list.fold(0, int.add)
}

pub fn do_it_again(input: String) -> Int {
  let #(left, right) =
    parse(input)
    |> list.unzip()

  left
  |> list.map(fn(a) { a * list.count(right, fn(b) { a == b }) })
  |> list.fold(0, int.add)
}

pub fn parse(input: String) -> List(#(Int, Int)) {
  string.split(input, "\n")
  |> list.map(fn(line) {
    let assert Ok(#(left, right)) = string.split_once(line, "   ")
    let assert Ok(a) = int.parse(left)
    let assert Ok(b) = int.parse(right)
    #(a, b)
  })
}

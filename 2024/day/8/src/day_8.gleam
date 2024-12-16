import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/set
import gleam/string
import simplifile.{read}

pub fn main() {
  let assert Ok(input) = read("./input")
  io.debug(do_it(string.trim(input)))
  io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  let #(antennas, max_x, max_y) = parse(input)

  list.group(antennas, fn(ant) {
    let #(_, _, kind) = ant
    kind
  })
  |> dict.values()
  |> list.flat_map(fn(group) {
    group
    |> list.combination_pairs()
    |> list.flat_map(fn(pair) {
      let #(#(x1, y1, _), #(x2, y2, _)) = pair
      let #(dx, dy) = #(x1 - x2, y1 - y2)

      [#(x1 + dx, y1 + dy), #(x2 - dx, y2 - dy)]
    })
  })
  |> set.from_list()
  |> set.filter(fn(item) {
    let #(x, y) = item
    x >= 0 && y >= 0 && x <= max_x && y <= max_y
  })
  |> set.size()
}

pub fn do_it_again(input: String) -> Int {
  let #(antennas, max_x, max_y) = parse(input)

  []
  |> list.length()
}

pub fn parse(input: String) -> #(List(#(Int, Int, String)), Int, Int) {
  let world =
    string.split(input, "\n")
    |> list.index_map(fn(line, y) {
      string.split(line, "")
      |> list.index_map(fn(item, x) { #(x, y, item) })
    })
    |> list.flatten()

  let max_x = list.fold(world, 0, fn(acc, a) { int.max(acc, a.0) })
  let max_y = list.fold(world, 0, fn(acc, a) { int.max(acc, a.1) })

  #(world |> list.filter(fn(item) { item.2 != "." }), max_x, max_y)
}

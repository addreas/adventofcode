import gleam/bool
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
  parse(input)
  |> list.filter(fn(level) { is_safe(level) })
  |> list.length()
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.filter(fn(level) {
    level
    |> list.combinations(list.length(level) - 1)
    |> list.any(is_safe)
  })
  |> list.length()
}

fn is_safe(level) -> Bool {
  let diffs =
    list.window_by_2(level)
    |> list.map(fn(ab) {
      let #(a, b) = ab
      a - b
    })

  bool.and(
    bool.or(list.all(diffs, fn(d) { d > 0 }), list.all(diffs, fn(d) { d < 0 })),
    bool.and(
      list.all(diffs, fn(d) { int.absolute_value(d) >= 1 }),
      list.all(diffs, fn(d) { int.absolute_value(d) <= 3 }),
    ),
  )
}

pub fn parse(input: String) -> List(List(Int)) {
  string.split(input, "\n")
  |> list.map(fn(line) {
    string.split(line, " ")
    |> list.map(fn(item) {
      let assert Ok(int) = int.parse(item)
      int
    })
  })
}

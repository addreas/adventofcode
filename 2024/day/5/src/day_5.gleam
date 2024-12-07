import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile.{read}

pub fn main() {
  let assert Ok(input) = read("./input")
  io.debug(do_it(string.trim(input)))
  io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  let #(rules, updates) = parse(input)

  updates
  |> list.filter(fn(update) {
    list.all(rules, fn(rule) {
      let #(a, b) = rule
      let #(x, y) =
        list.index_fold(update, #(-1, -1), fn(acc, item, index) {
          case item {
            x if x == a -> #(index, acc.1)
            x if x == b -> #(acc.0, index)
            _ -> acc
          }
        })
      x < y || x == -x || y == -1
    })
  })
  |> list.map(fn(update) {
    update
    |> list.drop(list.length(update) / 2)
    |> list.first()
    |> result.unwrap(0)
  })
  |> io.debug()
  |> list.fold(0, int.add)
}

pub fn do_it_again(input: String) -> Int {
  let #(rules, updates) = parse(input)
  // |> list.map(fn(_) { 0 })
  // |> list.fold(0, int.add)

  0
}

pub fn parse(input: String) -> #(List(#(Int, Int)), List(List(Int))) {
  let assert Ok(#(rules, updates)) = string.split_once(input, "\n\n")

  let rules =
    rules
    |> string.split("\n")
    |> list.map(fn(line) {
      let assert Ok(#(a, b)) = string.split_once(line, "|")
      let assert Ok(a) = int.parse(a)
      let assert Ok(b) = int.parse(b)
      #(a, b)
    })

  let updates =
    updates
    |> string.split("\n")
    |> list.map(fn(line) {
      string.split(line, ",")
      |> list.map(fn(item) {
        let assert Ok(int) = int.parse(item)
        int
      })
    })

  #(rules, updates)
}

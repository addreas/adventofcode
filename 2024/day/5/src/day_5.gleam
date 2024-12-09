import gleam/int
import gleam/io
import gleam/list
import gleam/order
import gleam/set.{type Set}
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
  |> list.filter(fn(update) { check_update(update, rules) })
  |> list.map(take_middle)
  |> list.fold(0, int.add)
}

fn take_middle(update) {
  let assert Ok(middle) =
    update
    |> list.drop(list.length(update) / 2)
    |> list.first()

  middle
}

fn rules_for_update(update) {
  // 3, 1, 7, 2, 5

  // 3|1, 3|7, 3|2, 3|5
  // 1|7, 1|2, 1|5
  // 7|2, 7|5
  // 2|5

  update
  |> list.index_map(fn(a, i) {
    update
    |> list.drop(i + 1)
    |> list.map(fn(b) { #(a, b) })
  })
  |> list.flat_map(fn(x) { x })
  |> set.from_list()
}

fn swap(ab) {
  let #(a, b) = ab
  #(b, a)
}

fn check_update(update, rules: Set(#(Int, Int))) {
  rules_for_update(update)
  |> set.map(swap)
  |> set.is_disjoint(rules)
}

fn fix_update(update: List(Int), rules: Set(#(Int, Int))) {
  list.sort(update, by: fn(a, b) {
    case set.contains(rules, #(a, b)), set.contains(rules, #(b, a)) {
      True, False -> order.Lt
      False, True -> order.Gt
      _, _ -> order.Eq
    }
  })
}

pub fn do_it_again(input: String) -> Int {
  let #(rules, updates) = parse(input)

  updates
  |> list.filter(fn(update) { !check_update(update, rules) })
  |> list.map(fn(update) {
    update
    |> fix_update(rules)
    |> take_middle()
  })
  |> list.fold(0, int.add)
}

pub fn parse(input: String) -> #(Set(#(Int, Int)), List(List(Int))) {
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
    |> set.from_list()

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

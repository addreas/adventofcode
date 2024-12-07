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
  // horizontal * 2, vertical * 2, diagonal * 4
  let words = parse(input)

  count(words)
  + count(words |> list.transpose())
  + count(
    words
    |> list.index_map(fn(row, i) { list.append(list.repeat("", i), row) })
    |> list.transpose(),
  )
  + count(
    words
    |> list.reverse()
    |> list.index_map(fn(row, i) { list.append(list.repeat("", i), row) })
    |> list.transpose(),
  )
}

fn count(words: List(List(String))) -> Int {
  words
  |> list.map(fn(row) {
    row
    |> list.window(4)
    |> list.count(fn(option) {
      option == ["X", "M", "A", "S"] || option == ["S", "A", "M", "X"]
    })
  })
  |> list.fold(0, int.add)
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.window(3)
  |> list.map(fn(window) {
    window
    |> list.transpose()
    |> list.window(3)
    |> list.count(fn(block) {
      case block {
        [["M", _, "M"], [_, "A", _], ["S", _, "S"]] -> True
        [["M", _, "S"], [_, "A", _], ["M", _, "S"]] -> True
        [["S", _, "M"], [_, "A", _], ["S", _, "M"]] -> True
        [["S", _, "S"], [_, "A", _], ["M", _, "M"]] -> True
        _ -> False
      }
    })
  })
  |> list.fold(0, int.add)
}

pub fn parse(input: String) -> List(List(String)) {
  string.split(input, "\n")
  |> list.map(fn(line) { string.split(line, "") })
}

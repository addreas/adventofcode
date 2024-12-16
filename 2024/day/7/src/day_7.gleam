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
  |> list.filter_map(fn(eq) {
    let #(res, operands) = eq
    let evaluations = eval(operands |> list.reverse())
    case list.contains(evaluations, res) {
      True -> Ok(res)
      False -> Error(Nil)
    }
  })
  |> list.fold(0, int.add)
}

fn eval(operands: List(Int)) -> List(Int) {
  case operands {
    [] -> []
    [x] -> [x]
    [x, ..rest] ->
      eval(rest)
      |> list.flat_map(fn(rest) { [x + rest, x * rest] })
  }
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.filter_map(fn(eq) {
    let #(res, operands) = eq
    let evaluations = eval2(operands |> list.reverse())
    case list.contains(evaluations, res) {
      True -> Ok(res)
      False -> Error(Nil)
    }
  })
  |> list.fold(0, int.add)
}

fn eval2(operands: List(Int)) -> List(Int) {
  case operands {
    [] -> []
    [x] -> [x]
    [x, ..rest] ->
      eval2(rest)
      |> list.flat_map(fn(rest) {
        let assert Ok(concat) =
          int.parse(int.to_string(rest) <> int.to_string(x))
        [x + rest, x * rest, concat]
      })
  }
}

pub fn parse(input: String) -> List(#(Int, List(Int))) {
  string.split(input, "\n")
  |> list.map(fn(line) {
    let assert Ok(#(left, right)) = string.split_once(line, ": ")
    let assert Ok(res) = int.parse(left)
    let operands =
      string.split(right, " ")
      |> list.map(fn(item) {
        let assert Ok(int) = int.parse(item)
        int
      })

    #(res, operands)
  })
}

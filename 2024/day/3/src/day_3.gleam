import gleam/int
import gleam/io
import gleam/list
import gleam/option.{Some}
import gleam/regexp
import gleam/string
import simplifile.{read}

pub fn main() {
  let assert Ok(input) = read("./input")
  io.debug(do_it(string.trim(input)))
  io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  parse(input)
  |> list.map(fn(instruction) {
    case instruction {
      Mul(a, b) -> a * b
      _ -> 0
    }
  })
  |> list.fold(0, int.add)
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.fold(#(True, 0), fn(state, instruction) {
    case instruction {
      Do -> #(True, state.1)
      Dont -> #(False, state.1)
      Mul(a, b) if state.0 -> #(True, state.1 + a * b)
      _ -> state
    }
  })
  |> fn(state) { state.1 }
}

pub type Instruction {
  Mul(Int, Int)
  Do
  Dont
}

pub fn parse(input: String) -> List(Instruction) {
  let assert Ok(re) =
    regexp.from_string(
      "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\)",
    )

  regexp.scan(re, input)
  |> list.map(fn(match) {
    case match {
      regexp.Match(content: _, submatches: [Some(a), Some(b)]) -> {
        let assert Ok(a) = int.parse(a)
        let assert Ok(b) = int.parse(b)
        Mul(a, b)
      }

      regexp.Match(content: "do()", submatches: []) -> Do
      regexp.Match(content: "don't()", submatches: []) -> Dont

      _ -> todo
    }
  })
}

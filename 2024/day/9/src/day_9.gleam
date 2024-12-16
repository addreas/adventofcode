import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/string
import simplifile.{read}

pub fn main() {
  let assert Ok(input) = read("./input")
  io.debug(do_it(string.trim(input)))
  io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  let source = parse(input)

  remap(source, list.reverse(source), [])
  |> list.index_map(fn(x, i) {
    case x {
      Some(x) -> x * i
      None -> 0
    }
  })
  |> list.fold(0, int.add)
}

fn remap(forward, reverse, res) {
  case forward, reverse {
    _, [None, ..rvs] -> remap(forward, rvs, res)
    [None, ..fwd], [Some(x), ..rvs] ->
      remap(
        fwd
          |> list.reverse()
          |> list.drop(1)
          |> list.drop_while(option.is_none)
          |> list.reverse(),
        rvs,
        [Some(x), ..res],
      )
    [Some(x), ..fwd], _ -> remap(fwd, reverse, [Some(x), ..res])

    _, _ -> list.reverse(res)
  }
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.map(fn(_) { 0 })
  |> list.fold(0, int.add)
}

pub fn parse(input: String) -> List(Option(Int)) {
  string.split(input, "")
  |> list.sized_chunk(2)
  |> list.index_map(fn(x, id) {
    case x {
      [file, empty] -> {
        let assert Ok(file_size) = int.parse(file)
        let assert Ok(empty_size) = int.parse(empty)

        #(id, file_size, empty_size)
      }
      [file] -> {
        let assert Ok(file_size) = int.parse(file)
        #(id, file_size, 0)
      }
      _ -> panic as "idk"
    }
  })
  |> list.flat_map(fn(item) {
    let #(id, file_size, empty_size) = item
    list.repeat(Some(id), file_size)
    |> list.append(list.repeat(None, empty_size))
  })
}

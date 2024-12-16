import gleam/erlang/process
import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import simplifile.{read}

pub fn main() {
  do_it_again(
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
  )
  |> io.debug()
  io.print("\n\n\n\n\n\n\n\n\n\n")
  // let assert Ok(input) = read("./input")
  // io.debug(do_it(string.trim(input)))
  // io.debug(do_it_again(string.trim(input)))
}

pub fn do_it(input: String) -> Int {
  let #(start, world) = parse(input)

  walk(start, Up, set.new(), world)
  |> set.map(fn(posd) { #(posd.0, posd.1) })
  |> set.size()
}

pub type Direction {
  Up
  Down
  Left
  Right
}

fn take_step(pos, dir) {
  let #(x, y) = pos
  case dir {
    Up -> #(x, y - 1)
    Down -> #(x, y + 1)
    Left -> #(x - 1, y)
    Right -> #(x + 1, y)
  }
}

pub fn turn(dir) {
  case dir {
    Up -> Right
    Right -> Down
    Down -> Left
    Left -> Up
  }
}

pub fn turn_left(dir) {
  case dir {
    Right -> Up
    Down -> Right
    Left -> Down
    Up -> Left
  }
}

pub fn walk(
  pos: #(Int, Int),
  dir: Direction,
  visited: Set(#(Int, Int, Direction)),
  world: World,
) {
  let visited = set.insert(visited, #(pos.0, pos.1, dir))
  let next = take_step(pos, dir)
  debug_print(visited, world)
  case set.contains(world.obstructions, next), pos {
    _, #(x, y) if x < 0 || y < 0 || x > world.max_x || y > world.max_y ->
      visited
    True, _ -> walk(pos, turn(dir), visited, world)
    False, _ -> walk(next, dir, visited, world)
  }
}

pub fn debug_print(visited, world: World) {
  list.range(0, world.max_y)
  |> list.map(fn(y) {
    list.range(0, world.max_x)
    |> list.map(fn(x) {
      case
        set.contains(world.obstructions, #(x, y)),
        set.contains(visited, #(x, y, Up))
        || set.contains(visited, #(x, y, Down)),
        set.contains(visited, #(x, y, Left))
        || set.contains(visited, #(x, y, Right))
      {
        True, _, _ -> "#"
        _, True, True -> "+"
        _, True, _ -> "|"
        _, _, True -> "-"
        _, _, _ -> "."
      }
    })
    |> string.join("")
  })
  |> string.join("\n")
  |> io.println()

  io.print("\u{001b}[" <> int.to_string(world.max_y + 1) <> "F")
  // process.sleep(100)
}

pub fn highlight_print(point, world: World) {
  list.range(0, world.max_y)
  |> list.map(fn(y) {
    list.range(0, world.max_x)
    |> list.map(fn(x) {
      case set.contains(world.obstructions, #(x, y)), #(x, y) == point {
        True, _ -> "#"
        _, True -> "O"
        _, _ -> "."
      }
    })
    |> string.join("")
  })
  |> string.join("\n")
  |> io.println()
}

pub fn print_obstructed(obstructed, world: World) {
  list.range(0, world.max_y)
  |> list.map(fn(y) {
    list.range(0, world.max_x)
    |> list.map(fn(x) {
      case
        set.contains(world.obstructions, #(x, y)),
        set.contains(obstructed, #(x, y))
      {
        True, _ -> "#"
        _, True -> "O"
        _, _ -> "."
      }
    })
    |> string.join("")
  })
  |> string.join("\n")
  |> io.println()

  obstructed
}

pub fn do_it_again(input: String) -> Int {
  let #(start, world) = parse(input)

  let original = walk(start, Up, set.new(), world)

  original
  |> set.filter(fn(pos) {
    let #(x, y, dir) = pos
    let exists = set.contains(original, #(x, y, turn_left(dir)))

    // case exists {
    //   True -> {
    //     io.println(
    //       "exists at ("
    //       <> int.to_string(x)
    //       <> ", "
    //       <> int.to_string(y)
    //       <> "), obstruct at ("
    //       <> int.to_string(nx)
    //       <> ", "
    //       <> int.to_string(ny)
    //       <> ")",
    //     )
    //     highlight_print(#(x, y), world)
    //     Nil
    //   }
    //   False -> Nil
    // }

    exists
  })
  |> set.map(fn(pos) {
    let #(x, y, dir) = pos
    let #(nx, ny) = take_step(#(x, y), dir)
    #(nx, ny)
  })
  |> set.difference(world.obstructions)
  |> print_obstructed(world)
  |> set.size()
}

pub type World {
  World(obstructions: Set(#(Int, Int)), max_x: Int, max_y: Int)
}

pub fn parse(input: String) -> #(#(Int, Int), World) {
  let assert #([#(x, y, _)], obstructions) =
    string.split(input, "\n")
    |> list.index_map(fn(line, y) {
      string.split(line, "")
      |> list.index_map(fn(item, x) { #(x, y, item) })
    })
    |> list.flatten()
    |> list.filter(fn(item) { item.2 != "." })
    |> list.partition(fn(item) { item.2 == "^" })

  let obstructions =
    obstructions
    |> list.map(fn(o) { #(o.0, o.1) })
    |> set.from_list()

  let max_x = set.fold(obstructions, 0, fn(acc, a) { int.max(acc, a.0) })
  let max_y = set.fold(obstructions, 0, fn(acc, a) { int.max(acc, a.1) })

  #(#(x, y), World(obstructions:, max_x:, max_y:))
}

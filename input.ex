#!/usr/bin/env elixir

Mix.install([
  {:req, "~> 0.2"},
  {:floki, "~> 0.30.0"}
])

year = Date.utc_today().year

day =
  case System.argv() do
    [day] -> day
    _ -> Date.utc_today().day
  end

path = "#{year}/day/#{day}"

IO.puts "Getting #{path}"

body =
  Req.get!("https://adventofcode.com/#{path}")
  |> Map.get(:body)
  |> Floki.parse_document!()

example_input =
  body
  |> Floki.find(~s{pre code})
  |> hd
  |> Floki.text()
  |> String.trim()

example_output =
  body
  |> Floki.find(~s{code em})
  |> List.last()
  |> Floki.text()
  |> String.trim()

session =
  case File.read(Path.expand("~/.aoc-session")) do
    {:ok, content} -> content
    _ -> IO.gets("session=")
  end
  |> String.trim()

code = """
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
  |> list.map(fn(_) { 0 })
  |> list.fold(0, int.add)
}

pub fn do_it_again(input: String) -> Int {
  parse(input)
  |> list.map(fn(_) { 0 })
  |> list.fold(0, int.add)
}

pub fn parse(input: String) -> List(List(Int)) {
  string.split(input, "\\n")
  |> list.map(fn(line) {
    string.split(line, " ")
    |> list.map(fn (item) {
      let assert Ok(int) = int.parse(item)
      int
    })
  })
}
"""

test_code = """
import gleam/io
import gleam/string
import gleeunit
import gleeunit/should

import day_#{day}.{do_it, do_it_again, parse}

pub fn main() {
  gleeunit.main()
}

const input = "
#{example_input}
"

pub fn parse_test() {
  input
  |> string.trim()
  |> parse()
  |> io.debug()
}

pub fn do_it_test() {
  input
  |> string.trim()
  |> do_it()
  |> should.equal(#{example_output})
}

pub fn do_it_again_test() {
  input
  |> string.trim()
  |> do_it_again()
  |> should.equal(#{example_output})
}
"""

File.mkdir_p("./#{path}")

System.cmd("gleam", ["new", ".", "--name", "day_#{day}", "--skip-github"], cd: "./#{path}")
System.cmd("gleam", ["add", "simplifile"], cd: "./#{path}")

code_path = "./#{path}/src/day_#{day}.gleam"
if !File.exists?(code_path) do
  File.write(code_path, code)
end

test_path = "./#{path}/test/day_#{day}_test.gleam"
if !File.exists?(test_path) do
  File.write(test_path, test_code)
end

Req.get!("https://adventofcode.com/#{path}/input", headers: %{Cookie: "session=#{session}"})
|> Map.get(:body)
|> then(&File.write("./#{path}/input", &1))
|> IO.inspect()

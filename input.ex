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
fn main() {
    let input = include_str!("../input");
    println!("{}", do_it(input.as_ref()));
    println!("{}", do_it_again(input.as_ref()));
}

fn do_it(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn do_it_again(input: &str) -> usize {
    parse(input).iter().map(|_| 0).sum()
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect()
}
#[test]
fn test() {
    let input = r"
        #{example_input |> String.replace("\n", "\n        ")}
    "
    .trim()
    .replace("        ", "");

    assert_eq!(do_it(input.as_str()), #{example_output});

    //assert_eq!(do_it_again(input.as_str()), 0);
}
"""

File.mkdir_p("./#{path}")

System.cmd("cargo", ["init", "--name", "day-#{day}"], cd: "./#{path}")

File.write("./#{path}/src/main.rs", code)

Req.get!("https://adventofcode.com/#{path}/input", headers: %{Cookie: "session=#{session}"})
|> Map.get(:body)
|> then(&File.write("./#{path}/input", &1))
|> IO.inspect()

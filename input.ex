#!/usr/bin/env elixir

Mix.install([
  {:req, "~> 0.2"},
  {:floki, "~> 0.30.0"}
])

year = Date.utc_today.year
day = case System.argv do
  [day] -> day
  _ -> Date.utc_today.day
end

body = Req.get!("https://adventofcode.com/#{year}/day/#{day}")
|> Map.get(:body)
|> Floki.parse_document!()


IO.puts "Example input:"

body
|> Floki.find(~s{pre code})
|> hd
|> Floki.text
|> String.trim
|> IO.puts

IO.puts "Expected result:"
body
|> Floki.find(~s{code em})
|> List.last
|> Floki.text
|> String.trim
|> IO.puts


session = case File.read(Path.expand("~/.aoc-session")) do
  {:ok, content} -> content
  _ -> IO.gets "session=" 
end
|> String.trim

File.mkdir_p "./#{year}/day/#{day}"

Req.get!("https://adventofcode.com/#{year}/day/#{day}/input", headers: %{Cookie: "session=#{session}"})
|> Map.get(:body)
|> then(&(File.write("./#{year}/day/#{day}/input", &1)))
|> IO.inspect

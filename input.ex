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
|> hd
|> Floki.text
|> String.trim
|> IO.puts


session = String.trim(IO.gets("session="))

Req.get!("https://adventofcode.com/#{year}/day/#{day}/input", headers: %{Cookie: "session=#{session}"})
|> Map.get(:body)
|> then(&(File.write("./#{year}/day/#{day}/input", &1)))
|> IO.inspect

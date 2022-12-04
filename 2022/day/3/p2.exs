File.read!("input.txt")
|> String.split("\n")
|> Enum.drop(-1)
|> Enum.map(&String.to_charlist/1)
|> Enum.map(&MapSet.new/1)
|> Enum.chunk_every(3)
|> Enum.map(fn [a, b, c] ->
  MapSet.intersection(a, b)
  |> MapSet.intersection(c)
  |> MapSet.to_list()
  |> hd
end)
|> Enum.map(fn
  x when x >= 97 and x <= 122 -> x - 97 + 1
  x when x >= 65 and x <= 90 -> x - 65 + 27
end)
|> Enum.sum()
|> IO.inspect()

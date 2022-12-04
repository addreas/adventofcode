File.read!("input.txt")
|> String.split("\n")
|> Enum.drop(-1)
|> Enum.map(&String.to_charlist/1)
|> Enum.map(&Enum.split(&1, div(length(&1), 2)))
|> Enum.map(fn {l, r} ->
  {MapSet.new(l), MapSet.new(r)}
end)
|> Enum.map(fn {l, r} ->
  MapSet.intersection(l, r)
  |> MapSet.to_list()
  |> hd
end)
|> Enum.map(fn
  x when x >= 97 and x <= 122 -> x - 97 + 1
  x when x >= 65 and x <= 90 -> x - 65 + 27
end)
|> Enum.sum()
|> IO.inspect()

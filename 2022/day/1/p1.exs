File.read!("input.txt")
|> String.split("\n\n")
|> Enum.map(fn x ->
  String.split(x)
  |> Enum.map(&String.to_integer/1)
  |> Enum.sum()
end)
|> Enum.max()
|> IO.inspect()

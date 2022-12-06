File.read!("input.txt")
# IO.gets("")
|> String.split("")
|> Enum.drop(1)
|> IO.inspect
|> Enum.chunk_every(14, 1)
|> IO.inspect
|> Enum.find_index(&(Enum.uniq(&1) |> Enum.count == 14))
|> then(&(&1 + 14))
|> IO.inspect
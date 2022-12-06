File.read!("input.txt")
# IO.gets("")
|> String.split("")
|> Enum.drop(1)
|> IO.inspect
|> Enum.chunk_every(4, 1)
|> IO.inspect
|> Enum.find_index(&(Enum.uniq(&1) |> Enum.count == 4))
|> then(&(&1 + 4))
|> IO.inspect
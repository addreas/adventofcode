moves =
  File.read!("input.txt")
  # File.read!("t1.txt")
  |> String.split("\n")
  |> Enum.drop(-1)
  |> Enum.map(fn l ->
    [dir, count] = String.split(l, " ")
    {dir, String.to_integer(count)}
  end)

snake_labels = "H0123456789" |> String.split("")

inspect_board = fn {snake, visited} -> 
  both = MapSet.union(MapSet.new(snake), visited)
  {maxx, _} = both |> Enum.max_by(fn {x, _} -> x end)
  {maxy, _} = both |> Enum.max_by(fn {_, y} -> y end)
  {_, minx} = both |> Enum.min_by(fn {x, _} -> x end)
  {_, miny} = both |> Enum.min_by(fn {_, y} -> y end)


  for y <- miny..maxy do
    row = for x <- minx..maxx do
      if {x, y} != {0, 0} do
          Enum.zip_with(snake, snake_labels)
          |> Enum.find(fn {point, _} -> point == {x, y} end, {nil,"."})
      else
        "s"
      end
    end

    IO.puts(Enum.join(row, ""))
  end

  IO.puts("")

  {snake, visited}
end

do_move = fn
  dir, {[head | tail], visited} -> {[head | tail], visited}
end

moves
|> Enum.reduce({1..10 |> Enum.map(fn _ -> {0, 0} end), MapSet.new()}, fn {dir, count}, acc ->
  IO.puts("== #{dir} #{count} ==")
  Enum.reduce(1..count, acc, fn _, acc -> do_move.(dir, acc) end)
end)
|> then(fn {_, _, visited} -> visited end)
|> Enum.count()
|> IO.inspect()

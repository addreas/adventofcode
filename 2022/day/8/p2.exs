board =
  File.read!("input.txt")
  |> String.split("\n")
  |> Enum.drop(-1)
  |> Enum.map(fn l ->
    String.split(l, "")
    |> Enum.drop(1)
    |> Enum.drop(-1)
    |> Enum.map(&String.to_integer/1)
  end)

view_from = fn x, y ->
  {left, right} = board |> Enum.at(y) |> Enum.split(x)
  {up, down} = board |> Enum.zip_with(& &1) |> Enum.at(x) |> Enum.split(y)
  middle = hd(right)

  [
    up |> Enum.reverse(),
    down |> Enum.drop(1),
    left |> Enum.reverse(),
    right |> Enum.drop(1)
  ]
  |> Enum.map(fn l ->
    case Enum.find_index(l, &(&1 >= middle)) do
      nil -> Enum.count(l)
      i -> i + 1
    end
  end)
  |> Enum.filter(&(&1 > 0))
  |> Enum.product()
end

for x <- 0..(length(board) - 1),
    y <- 0..(length(board) - 1) do
  view_from.(x, y)
end
|> Enum.max()
|> IO.inspect()

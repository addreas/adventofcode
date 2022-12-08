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

scan = fn board ->
  top = Enum.at(board, 0)
  truth = Enum.map(board, &(&1 == &1))

  board
  |> Enum.drop(1)
  |> Enum.map_reduce(
    top,
    fn row, top ->
      {
        Enum.zip_with(row, top, &Kernel.>/2),
        Enum.zip_with(row, top, &max/2)
      }
    end
  )
  |> then(fn {b, _} -> [truth | b] end)
end

boards = [
  board
  |> scan.(),
  board
  |> Enum.reverse()
  |> scan.()
  |> Enum.reverse(),
  board
  |> Enum.zip_with(& &1)
  |> scan.()
  |> Enum.zip_with(& &1),
  board
  |> Enum.zip_with(& &1)
  |> Enum.reverse()
  |> scan.()
  |> Enum.reverse()
  |> Enum.zip_with(& &1)
]

boards
|> Enum.reduce(fn board, acc ->
  Enum.zip_with(board, acc, fn r1, r2 -> Enum.zip_with(r1, r2, &Kernel.||/2) end)
end)
|> Enum.map(fn row -> Enum.count(row, & &1) end)
|> Enum.sum()
|> IO.inspect()

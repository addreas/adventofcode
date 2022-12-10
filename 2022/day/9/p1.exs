moves =
  File.read!("input.txt")
  # File.read!("t1.txt")
  |> String.split("\n")
  |> Enum.drop(-1)
  |> Enum.map(fn l ->
    [dir, count] = String.split(l, " ")
    {dir, String.to_integer(count)}
  end)

inspect_board = fn {head, tail, visited} -> 
  {maxx, _} = visited |> Enum.max_by(fn {x, _} -> x end)
  {maxy, _} = visited |> Enum.max_by(fn {_, y} -> y end)
  {_, minx} = visited |> Enum.min_by(fn {x, _} -> x end)
  {_, miny} = visited |> Enum.min_by(fn {_, y} -> y end)

  for y <- miny-1..maxy do
    IO.puts(for x <- minx..maxx+1 do
      cond do
        {x, y} == head -> "H"
        {x, y} == tail -> "T"
        {x, y} == {0, 0} -> "s"
        MapSet.member?(visited, {x, y}) -> "#"
        true -> "."
      end
    end
    |> Enum.join(""))
  end

  IO.puts("")

  {head, tail, visited}
end

do_move = fn dir, {{hx, hy}, {tx, ty}, visited} ->
  {hx2, hy2} =
    case dir do
      "U" -> {hx, hy - 1}
      "D" -> {hx, hy + 1}
      "L" -> {hx - 1, hy}
      "R" -> {hx + 1, hy}
    end

  {tx2, ty2} =
    case {dir, tx, ty} do
      {_, ^hx, ^hy} -> {tx, ty} # were stacked, stay behind
      {_, ^hx2, ^hy2} -> {tx, ty} # now stacked, stay behind

      {"U", _, ^hy} -> {tx, ty} # going up, were in the same row, stay behind
      {"D", _, ^hy} -> {tx, ty} # going down, were in the same row, stay behind
      {"L", ^hx, _} -> {tx, ty} # going left, were in the same col, stay behind
      {"R", ^hx, _} -> {tx, ty} # going right, were in the same col, stay behind

      {"U", _, ^hy2} -> {tx, ty} # going up, returning to the same row, stay
      {"D", _, ^hy2} -> {tx, ty} # going up, returning to the same row, stay
      {"L", ^hx2, _} -> {tx, ty} # going right, returning to the same col, stay
      {"R", ^hx2, _} -> {tx, ty} # going right, returning to the same col, stay

      {"U", _, _} -> {hx2, ty - 1} # going up, move to same col, go up
      {"D", _, _} -> {hx2, ty + 1} # going down, move to same col, move down
      {"L", _, _} -> {tx - 1, hy2} # going left, move to same row, move left
      {"R", _, _} -> {tx + 1, hy2} # going right, move to same row, mote right
    end

  {{hx2, hy2}, {tx2, ty2}, MapSet.put(visited, {tx2, ty2})}
end

moves
|> Enum.reduce({{0, 0}, {0, 0}, MapSet.new([{0,0}])}, fn {dir, count}, acc ->
  IO.puts("== #{dir} #{count} ==")
  Enum.reduce(1..count, acc, fn _, acc -> do_move.(dir, acc) end)
end)
|> then(fn {_, tail, visited} -> MapSet.put(visited, tail) end)
|> Enum.count()
|> IO.inspect()

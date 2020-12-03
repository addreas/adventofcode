defmodule Day2 do
  def check_line(right, {line, index}) do
    pos = Integer.mod(index * right, String.length(line))
    String.at(line, pos) == "#"
  end

  def get_slope(down, right) do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.take_every(down)
    |> Stream.with_index()
    |> Stream.filter(&check_line(right, &1))
    |> Enum.count()
  end
end

results =
  for {down, right} <- [{1, 1}, {1, 3}, {1, 5}, {1, 7}, {2, 1}] do
    IO.inspect(Day2.get_slope(down, right))
  end

IO.inspect(Enum.reduce(results, &:erlang.*/2))

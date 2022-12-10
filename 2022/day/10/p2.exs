File.read!("input.txt")
# File.read!("t1.txt")
# File.read!("t2.txt")
  |> String.split("\n")
  |> Enum.drop(-1)
  |> Enum.flat_map(fn l ->
    case  String.split(String.trim(l), " ") do
      ["addx", count] ->  [:addx, {:addx, String.to_integer(count)}]
      ["noop"] -> [:noop]
    end
  end)
  |> Enum.scan(1, fn
    {:addx, n}, x -> x + n
    _, x -> x
  end)
  |> Enum.chunk_every(40)
  |> Enum.map(fn row ->
    Enum.with_index(row, 1)
    |> Enum.map(fn {sprite_middle, current_pixel} ->
      if abs(sprite_middle - current_pixel) <= 1 do
        "#"
      else
        "."
      end
    end)
    |> Enum.join()
  end)
  |> Enum.join("\n")
  |> IO.puts

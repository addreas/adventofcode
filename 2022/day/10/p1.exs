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
  |> Enum.with_index(2)
  |> Enum.drop(18)
  |> Enum.take_every(40)
  |> Enum.map(fn {c, x} -> c * x end)
  |> Enum.sum
  |> IO.inspect
  

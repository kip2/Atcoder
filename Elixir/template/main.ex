defmodule Main do
  def main do
    [a, b, c] =
      IO.gets("") |> String.trim() |> String.split() |> Enum.map(&String.to_integer/1)

    result = solve()
    IO.puts(result)
  end

  def solve() do
    ""
  end
end

Main.main()

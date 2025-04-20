defmodule Main do
  def main do
    IO.gets("")
    |> String.trim()
    |> solve()
    |> IO.puts()
  end

  def solve(str) do
    String.graphemes(str)
    |> then(fn [first | rest] -> Enum.join(rest) <> first end)
  end
end

Main.main()

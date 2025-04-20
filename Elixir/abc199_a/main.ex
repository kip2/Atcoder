defmodule Main do
  def main do
    [a, b, c] =
      IO.gets("") |> String.trim() |> String.split() |> Enum.map(&String.to_integer/1)

    result = solve(a, b, c)
    IO.puts(result)
  end

  def solve(a, b, c) do
    if a * a + b * b < c * c do
      "Yes"
    else
      "No"
    end
  end
end

Main.main()

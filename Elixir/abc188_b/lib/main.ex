defmodule Main do
  def main do
    _ = InputHelper.read_line()
    a_list = InputHelper.read_integer_list()
    b_list = InputHelper.read_integer_list()

    result = solve(a_list, b_list)
    IO.puts(result)
  end

  def solve(a_list, b_list) do
    Enum.zip(a_list, b_list)
    |> Enum.map(fn {x, y} -> x * y end)
    |> Enum.sum()
    |> then(fn
      0 -> "Yes"
      _ -> "No"
    end)
  end
end

defmodule InputHelper do
  def read_line() do
    IO.read(:line)
  end

  def read_integer() do
    IO.read(:line) |> String.trim() |> String.to_integer()
  end

  def read_integer_list() do
    IO.read(:line)
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end
end

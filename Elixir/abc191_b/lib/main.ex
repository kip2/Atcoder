defmodule Main do
  def main do
    [_ | [x]] = InputHelper.read_to_list()

    InputHelper.read_to_list()
    |> Enum.reject(fn y -> y == x end)
    |> OutputHelper.to_output_format_list()
    |> IO.puts()
  end
end

defmodule OutputHelper do
  def to_output_format_list(list) do
    list |> Enum.intersperse(" ") |> IO.iodata_to_binary()
  end
end

defmodule InputHelper do
  def read_to_list() do
    IO.read(:line) |> String.trim() |> String.split()
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

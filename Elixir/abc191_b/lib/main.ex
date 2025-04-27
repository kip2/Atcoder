defmodule Main do
  def main do
    [_ | [x]] = IO.read(:line) |> String.trim() |> String.split(" ")

    IO.read(:line)
    |> String.trim()
    |> String.split(" ")
    |> Enum.reject(fn y -> y == x end)
    |> to_output_format_list()
    |> IO.puts()
  end

  def to_output_format_list(list) do
    list |> Enum.join(" ")
  end
end

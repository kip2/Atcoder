defmodule Main do
  def main do
    result = solve()
    IO.puts(result)
  end

  def solve() do
    ""
  end
end

defmodule InputHelper do
  @doc """
  Get an integer from user input.

  ## Parameters
  None

  ## Return value
  - An integer value received from standard input.
  """
  def read_integer() do
    IO.gets("") |> String.trim() |> String.to_integer()
  end

  @doc """
  Get a list of integers from standard input.

  ## Parameters
  None

  ## Return value
  - A list of integers recieved from standard input.
  """
  def read_integer_list() do
    IO.gets("")
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end
end

defmodule RotationalCipher do
  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    text
    |> to_charlist
    |> Enum.map(fn code ->
      shift_char(code, shift)
    end)
    |> to_string
  end

  @spec shift_char(code :: char, shift :: integer) :: char
  defp shift_char(code, shift) when code in ?a..?z do
    rem((code - ?a + shift), 26) + ?a
  end

  defp shift_char(code, shift) when code in ?A..?Z do
    rem(code - ?A + shift, 26) + ?A
  end

  defp shift_char(code, _), do: code
end

defmodule Words do
  @doc """
  Count the number of words in the sentence.

  Words are compared case-insensitively.
  """
  @spec count(String.t()) :: map
  def count(sentence) do
    sentence
    |> String.downcase
    |> tokenise
    |> Enum.reduce(%{}, fn ([word], words) -> 
      Map.update words, word, 1, &(&1 + 1)
    end)
  end

  @spec tokenise(String.t()) :: [String.t()]
  defp tokenise(sentence) do
    Regex.scan ~r/(*UTF)[\p{L}0-9-]+/i, sentence
  end
end

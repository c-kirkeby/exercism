defmodule Bob do
  def hey(input) do
    cond do
      yelling?(input) && question?(input) -> "Calm down, I know what I'm doing!"
      question?(input)                    -> "Sure."
      say_nothing?(input)                 -> "Fine. Be that way!"
      yelling?(input)                     -> "Whoa, chill out!"
      true                                -> "Whatever."
    end
  end

  def yelling?(input) do
    String.upcase(input) == input && String.downcase(input) != input
  end

  def question?(input) do
    String.ends_with?(input, "?")
  end

  def say_nothing?(input) do
    String.trim(input) == ""
  end
end

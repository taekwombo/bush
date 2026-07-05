defmodule HophopTest do
  use ExUnit.Case
  doctest Hophop

  test "greets the world" do
    assert Hophop.hello() == :world
  end
end

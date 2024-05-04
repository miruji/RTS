##
  ab

  **
    ** lower first char
    name
    parameters
      final Int a -> = const first a value
      final Int b -> = const first b value
    return type
      Int
    content
      a + b
##

ab(final Int a, final Int b) -> Int
  = a + b

c = ab(10, 20)
println(c)
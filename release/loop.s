##
  loop

  kinds and declarate:
    loop         = while true
      next lines

    loop true    = while true
      next lines

    loop a = 10  = while a = 10
      next lines

    loop a = 10; a < 10; i++
      next lines
    ** to:do: rework "for" syntax
##

a = [10,20,30]
loop l in a
  println(l)
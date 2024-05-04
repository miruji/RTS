##
  if-else

  kinds:
    if
    ef -> else if
    el -> else

    = (== not allowed)

  declarate kinds:
    if condition
      next lines

    if condition
      next lines
    ef condition
      enxt lines

    if condition
      next lines
    el
      next lines

    if condition
      next lines
    ef condition
      next lines
    el
      next lines
##

a = 2
if a = 0
  println("false")
ef a = 1
  println("true")
el
  println("else")
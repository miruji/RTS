#
  variable

  memoryCellName   -> final    locked
  memoryCellName~  -> variable locked
  memoryCellName~~ -> variable unlocked

#
  a            # locked final
  a = 10       # now a is a constant Unsigned

  b: UInt = 20 # final locked-Unsigned
               # negative values will not be able 
               # to change the type to Integer

  c~           # var
  c = a + b    # now c is a locked-Unsigned
               # negative values will not be able 
               # to change the type to Integer

  d~~ = c      # var unlocked-Unsigned
  d -= 31      # now d is a Integer, = -1

a = 10
b = 20
c = (a+b)*0.5
println(c-0.5) # 14.5

println(1) # next todo
  a~ = 5
  println(--a) # now -, out 4
  println(a--) # out 4, now 3
  println(++a) # now +, out 4
  println(a++) # out 4, now 5
  println(a)   # out 5
println(2)

# todo ++ -- ** // %% ^^
# todo %= ^=
# todo print
# todo print-println multi-arg
# todo String in print-println
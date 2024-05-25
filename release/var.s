##
  variable

  variableName   -> final    locked
  variableName~  -> variable locked
  variableName~~ -> variable unlocked
##

##
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
##

#a~ = 30 # todo: add no create memory cell check
a~ = (10+(20+(30))*2)/10 # 11
a -= 100                 # -89
a += 11                  # -78
a *= 2                   # -156
a /= 2                   # -78
a *= -1                  # 78
a /= -1                  # -78
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

a~ = 10
a -= (((10+(10+20)*3+10)*2)/10/2-10+2)/3-1 # 0
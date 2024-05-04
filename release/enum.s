##
  enum

  declarate: 
    ** upper first char
    EnumName
      ** next line + tab+1
      content

  operators:
    = or !=

  appeal:
    EnumName:itemName
    ** to:do: itemName or ItemName or item_name ?
##

HumanType
  None       # 0
  Man        # 1
  Woman      # 2

a = HumanType:Man
b = HumanType:Woman

if a != b and a != HumanType:None
  println("the Man!")
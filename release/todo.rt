a = 100
# next comes the block
 
  b = 0  # create
  a = 0  # 100 -> 10
  ? a <= 10, a++
    println(a)
    ? a = 10 && b < 2, b++
      println("  ",b)
      a = 0
      :2  # go 2 blocks up
    :1    # go 1 blocks up
  # if a > 10 then 
  ? a < 15, a++
    ? a = 14
      :x 2  # exit 2 blocks
    :1      # go 1 blocks up
# no b
println(a) # 14
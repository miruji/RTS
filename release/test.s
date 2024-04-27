##
  test script
  * can use argc/argv vars
  * main function doesn't make sense
##
argv_len = argv.len()
println(argc, argv.len())

c = if argc == argv_len
  20
else
  30
println("c = "+c)
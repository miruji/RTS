##
  test
##
main(final Int argc, final String[] argv)
  argv_len = argv.len()
  println(argc, argv.len())

  c = if argc == argv_len
    20
  else
    30
  println("c = "+c)
##
  test text
  test comment
##
Human
  public age
  constr(age)
    this.age = age
  destr
    println("Deleted")
  public getAge()
    ret age
main
  println("Hello world!")
  h = new Human(32)
  for i = 0; i < 10; i++
    println(i)
  prinln("Program end: "+h.getAge())
  # next ret 0
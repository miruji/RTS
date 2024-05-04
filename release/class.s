##
  class

  struct
    ** upper first char
    Human
      age    (public var)
      constr (not   class name)
      destr  (not ~ class name)
      getAge (function)
##

Human
  public age
  constr(age)
    this.age = age
  destr
    println("Deleted")
  public getAge
    = age

h = new Human(32)
prinln("Human age: "+h.getAge())
# next ret 0 and print "Deleted"
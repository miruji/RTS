##
  class
##

Human                  # if upper first char + consr procedure
  age: U               # private final locked-Unsigned
  public ~~money: U    # public var unlocked-Unsigned, type may change
  constr(age: U)       # constructor procedure <- Unsigned final age
    this.age = age     # now Human.age is a constant
  destr                # destructor
    println("Deleted")
  public getAge        # public function, no param
    = age              # return Unsigned value

h = new Human(32)                # final Human
h.money = -100                   # type became Integer, oh no!
prinln("Human age: "+h.getAge()) # next = 0 and println "Deleted"
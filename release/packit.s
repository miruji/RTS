#
  packet kit for ArchLinux

? argc = 0
  println("packit: no arguments")
  exit(0)

#
# sync package (update/download)
? argv[0] = "s"
  ? argc = 2
    println("packit: update "+argv[1])
    exec("sudo pacman --noconfirm -S archlinux-keyring")
    exec("sudo pacman --noconfirm -S "+argv[1])
  ?
    println("no 2 argument")
# sync all
? argv[0] = "sa"
  println("packit: update all packages")
  exec("sudo pacman --noconfirm -S archlinux-keyring")
  exec("sudo pacman --noconfirm -Syu")
#
# remove package
? argv[0] = "r"
  ? argc = 2
    println("packit: remove package")
    exec("sudo pacman --noconfirm -R"+argv[1])
  ?
    println("no 2 argument")
# remove package with dependency
? argv[0] = "rd"
  ? argc = 2
    println("packit: remove package with dependency")
    exec("sudo pacman --noconfirm -Rn "+argv[1])
  ?
    println("no 2 argument")
# remove package with dependency, configuration
? argv[0] = "rdc"
  ? argc = 2
    println("packit: remove package with dependency, configuration")
    exec("sudo pacman --noconfirm -Rns "+argv[1])
  ?
    println("no 2 argument")
#
# litter packages
? argv[0] = "l"
  println("packit: show litter packages")
  exec("pacman -Qdtq")
# clear litter packages
? argv[0] = "lc"
  println("packit: clear litter packages")
  exec("sudo pacman --noconfirm -Rdd $(pacman -Qdtq)")
  # todo: check litter list
#
# clear cache, leaving latest
? argv[0] = "c"
  println("packit: clear cache, leaving latest")
  exec("sudo pacman --noconfirm -Sc")
# clear cache all
? argv[0] = "ca"
  println("packit: clear cache, all")
  exec("sudo pacman --noconfirm -Scc")
#
# bad command
?
  println("bad command, no work")
#
  packet kit for ArchLinux

println("spl argc: "+str(argc))
? argv[0] = "s"
  println("update "+argv[1])
  exec("sudo pacman -S --noconfirm "+argv[1])
? argv[0] = "sa"
  println("update all")
  exec("sudo pacman -Syu --noconfirm")
? argv[0] = "sk"
  println("update keyring")
  exec("sudo pacman -S --noconfirm archlinux-keyring")
?
  println("no work")
#!/bin/bash
# RTS run file script for tests
#
#        x    32/64  debug    file
# e:  ./run.sh  64     0     test.rt  # no debug
# e:  ./run.sh  64     1     test.rt  # drun
# e:  ./run.sh  64     2     test.rt  # drun + build debug
clear

# remove back
if [ -e "rts" ]; then
  rm -rf ./rts
fi

# debug mode
debugLevel=$2

# build
if [ "$debugLevel" -eq 2 ]; then
  ./build.sh $1 true
else
  ./build.sh $1 false
fi

if [ $? -ne 0 ]; then
  echo "[run] Skipped"
  exit 1
fi

# run file based on debug level
case $debugLevel in
  0)
    ./rts run "${@:3}"
    ;;
  1)
    ./rts drun "${@:3}"
    ;;
  2)
    ./rts drun "${@:3}"
    ;;
  *)
    echo "Invalid debug level. Use 0, 1, or 2."
    exit 1
    ;;
esac

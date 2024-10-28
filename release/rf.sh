#!/bin/bash
# RTS run file script for tests
#
#        x   32/64  file
# e:  ./rf.sh  64  test.rt
clear

# remove back
if [ -e "rts" ]; then
  rm -rm ./rts
fi

# debug mode
argcBegin=2
debug=false
for arg in "$@"
do
  if [[ "$arg" == "d" ]]; then
      debug=true
      argcBegin=3
  fi
done

# build
./build.sh $1 $debug
if [ $? -ne 0 ]; then
  echo "[rf] Skipped"
  exit 1
fi

# run file
if [ "$debug" == "true" ]; then
  ./rts -rf "${@:$argcBegin}" -d
else
  ./rts -rf "${@:$argcBegin}"
fi

#!/bin/bash

clear

# remove back
if [ -e "spl" ]; then
  rm -rm ./spl
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
  echo "rf: Skipped"
  exit 1
fi

# run file
if [ "$debug" == "true" ]; then
  ./spl -rf "${@:$argcBegin}" -d
else
  ./spl -rf "${@:$argcBegin}"
fi

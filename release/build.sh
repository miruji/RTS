#!/bin/bash
# build spl x86-32 or x86-64
# and optimize (strip + upx)

clear

# build file
cd ../
echo -n "build: "
# x86-64
if [ "$1" == "64" ]; then
	builded=true
	script -q -e -c 'cargo build --release' ./log > /dev/null 2>&1
	if [ $? -ne 0 ]; then
		builded=false
	fi

	sed -i -e '1d;$d' -e '/^$/d' ./log  # remove begin-end, end space line

	#
	if [ "$builded" == "false" ]; then
		echo ""
		cat ./log  # output log
		exit 1
	fi
	if [[ "$2" == "true" ]]; then
		echo ""
		cat ./log  # output log
	fi
	ls &>> ./log
	rm -f  ./log  # remove log
# x86-32
elif [ "$1" == "32" ]; then
	builded=true
	script -q -e -c 'cargo build --release --target=i686-unknown-linux-gnu' ./log > /dev/null 2>&1
	if [ $? -ne 0 ]; then
		builded=false
	fi

	sed -i -e '1d;$d' -e '/^$/d' ./log  # remove begin-end, end space line

	# if build error
	if [ "$builded" == "false" ]; then
	    rustup target add i686-unknown-linux-gnu &> ./log2
	    # if no rustup or i686-unknown-linux-gnu
	    if [ $? -ne 0 ]; then
	    	echo "You need to install 'i686-unknown-linux-gnu' using rustup"
			rm -f ./log2  # remove log2
			rm -f ./log   # remove log
	    	exit 1
		fi
		ls &>> ./log2
		rm -f  ./log2  # remove log2

		cat   ./log  # output log
		rm -f ./log  # remove log
		exit 1
	fi
	#
	if [ "$2" == "true" ]; then
		echo ""
		cat ./log  # output log
	fi
	ls &>> ./log
	rm -f  ./log  # remove log
fi
echo "Everything is fine linux-x86-$1"

splPath="target/release/spl"
if [ "$1" == "32" ]; then
	splPath="target/i686-unknown-linux-gnu/release/spl"
fi

if [ -e "$splPath" ]; then
	# optimize
	strip $splPath
	upx --best -q -q -q $splPath

	# move here
	mv $splPath release
fi

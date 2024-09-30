#!/bin/bash
for (( num=0xe0000000; num <= 0xe1111111; num+=1024 ))
do
	hex=$(printf "%x\n" $num)
	cargo run -q -- -s /dev/ttyACM0 -c "$hex"
	echo $hex
	#sleep 0.1
done

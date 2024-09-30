#!/bin/bash
while true
do
	cargo run -q -- -s /dev/ttyACM0 -c "#00ff00"
	sleep 0.1
	cargo run -q -- -s /dev/ttyACM0 -c "#ffffff"
	sleep 0.1
	cargo run -q -- -s /dev/ttyACM0 -c "#ff0000"
	sleep 0.1
done

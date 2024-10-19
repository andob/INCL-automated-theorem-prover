#!/bin/bash
set -e #fail on first error
#while true
#do
	cargo bench
	cat data.csv >> data.csv.bak
#done

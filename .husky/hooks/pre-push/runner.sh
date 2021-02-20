#!/bin/bash

set -eu

cd $(dirname $0)



procs=()
for entry in ./*
do
	script="$(basename $entry)"
	if [ "$(basename $0)" != "$script" ]; then
		echo "start run:$script"
		./$script &
		procs=("${procs[@]}" $!)
	fi
done


for proc in ${procs[@]};
do
	wait $proc
done

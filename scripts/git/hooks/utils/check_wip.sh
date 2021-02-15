#!/bin/bash

set -eu

branch_name="$(git symbolic-ref --short HEAD)"

case $branch_name in
	wip/*) exit 0;;

esac

exit 1

#!/bin/bash

set -eu
file_path=$1

git add $file_path
if [ "$(git diff --name-only --cached)" == "" ]; then
	exit 1
fi

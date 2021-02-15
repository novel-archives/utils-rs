#!/bin/bash

set -eu

wd=$(dirname $0)/../../../..
cd $wd

if $wd/scripts/git/hooks/utils/check_wip.sh; then
	exit 0
fi

RUSTFLAGS="-D warnings" cargo clippy

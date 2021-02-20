#!/bin/bash

set -eu


cd $(dirname $0)/../../..
wd=$(pwd)

if $wd/.husky/hooks/utils/is_remote_branch_wip.sh; then
	exit 0
fi


cargo make pre-push

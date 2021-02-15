#!/bin/bash

set -eu
wd=$(dirname $0)/../../../..
cd $wd
file_path=$1

rustfmt $file_path
$wd/scripts/git/hooks/utils/after_format.sh $file_path

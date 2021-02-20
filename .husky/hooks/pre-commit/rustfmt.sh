#!/bin/bash

set -eu
wd=$(dirname $0)/../../..
cd $wd
file_path=$1

 cargo fmt -- $file_path

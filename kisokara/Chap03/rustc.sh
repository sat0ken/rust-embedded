#!/bin/bash
file=$1
rustc $file

file=$(basename $file)
file=$(echo ${file%.*})

./$file

rm $file

#!/bin/bash

dir="tmp"
if [ ! -d $dir ]; then
    mkdir tmp
fi

base_filename="file"
for ((i=1; i<=100; i++))
do
    filename="${base_filename}${i}.txt"
    touch "$dir/$filename" > /dev/null
done

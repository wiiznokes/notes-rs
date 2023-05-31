#!/bin/bash

clear

cargo build --release

if [[ $? -eq 0 ]]; then

    #clear
    ./target/release/notes_rs /home/lenaic/Documents/notes-rs/aaa_test/
fi
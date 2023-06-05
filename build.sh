#!/bin/bash
set -e
clear

cargo build --release

./target/release/notes_rs /home/lenaic/Documents/notes-rs/aaa_test


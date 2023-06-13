#!/bin/bash
set -e
clear

cargo build --release

./target/release/notes_rs ./tmp


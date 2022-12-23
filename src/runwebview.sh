#!/bin/bash

echo "inside runwebview.sh"
echo "First Parameter of the script is $1"

cd /Users/peterweyand/Code/rustprojects/project1_2/webview
cargo build && cargo run -- $1
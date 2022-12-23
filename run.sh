#!/bin/bash

RUSTFLAGS="$RUSTFLAGS -A warnings" cargo build && RUSTFLAGS="$RUSTFLAGS -A warnings" cargo run 
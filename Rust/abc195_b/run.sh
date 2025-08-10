#!/bin/bash

cargo build --release
oj test -c ./target/release/$(basename $(pwd))

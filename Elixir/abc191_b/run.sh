#!/bin/bash

# compile
mix compile

# execute oj t
oj test -d sample -c "mix run -e Main.main"

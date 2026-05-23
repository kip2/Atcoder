#!/bin/bash

cd "$(dirname "$0")" || exit 1

dmd -unittest -run main.d

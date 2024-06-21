#!/bin/sh

cargo compete new $1

# echo "$1/*" >> .gitignore
cargo member include $1

cd $1

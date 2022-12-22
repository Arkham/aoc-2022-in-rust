#! /usr/bin/env bash

# part 1
# swipl \
#   -g "use_module(library(clpr))" \
#   -g "{$(sed -r -e 's/\<./\U&/g' -e 's/: / = /g' "$1" | paste -sd "," -)}, write(Root)" \
#   -g halt

swipl \
  -g "use_module(library(clpr))" \
  -g "{$(sed -r -e '/^humn/d' -e 's/^root: (\w+) . (\w+)$/\1 = \2/' -e 's/\<./\U&/g' -e 's/: / = /g' "$1" | paste -sd "," -)}, write(Humn)" \
  -g halt

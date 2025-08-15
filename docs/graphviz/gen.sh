#!/bin/bash
gvpack -u "$1" | dot -Gdpi=500 -Granksep=0.4 -Nheight=0.4 -Nshape=plaintext -Earrowhead=vee -Earrowsize=0.5 -Tpng -o "$1.png"
convert "$1.png" "$1.bmp"
potrace -s "$1.bmp"
inkscape --export-plain-svg --export-filename="$1.svg" --export-area-drawing "$1.svg"
rm "$1.bmp"

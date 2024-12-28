#!/bin/bash
gvpack -u "$1" | dot -Tpng -Nshape=plaintext -Earrowhead=none -o "$1.png"

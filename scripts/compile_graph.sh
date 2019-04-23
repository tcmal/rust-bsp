#!/usr/bin/sh
rm -f ./structure.png
dot -ostructure.png -Tpng structure.dot

#!/bin/sh
# Copyright (C) 2019 Oscar Shrimpton
# 
# This file is part of stockton-bsp.
# 
# stockton-bsp is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# stockton-bsp is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with stockton-bsp.  If not, see <http://www.gnu.org/licenses/>.

rm tests/tree/test_tree.bin
node scripts/compile_hex.js tests/tree/test_tree.hex
rm tests/brushes/test_brushes.bin
node scripts/compile_hex.js tests/brushes/test_brushes.hex
rm tests/effects/test_effects.bin
node scripts/compile_hex.js tests/effects/test_effects.hex
rm tests/vertices/test_vertices.bin
node scripts/compile_hex.js tests/vertices/test_vertices.hex
rm tests/vertices/test_meshvertices.bin
node scripts/compile_hex.js tests/vertices/test_meshvertices.hex
rm tests/lightmaps/test_lightmaps.bin
node scripts/compile_hex.js tests/lightmaps/test_lightmaps.hex
rm tests/visdata/test_visdata.bin
node scripts/compile_hex.js tests/visdata/test_visdata.hex
rm tests/models/test_models.bin
node scripts/compile_hex.js tests/models/test_models.hex
rm tests/advertisements/test_advertisements.bin
node scripts/compile_hex.js tests/advertisements/test_advertisements.hex
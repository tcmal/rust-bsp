#!/usr/bin/node
// Copyright (C) 2019 Oscar Shrimpton
// 
// This file is part of stockton-bsp.
// 
// stockton-bsp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// stockton-bsp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with stockton-bsp.  If not, see <http://www.gnu.org/licenses/>.


var fs = require('fs');
var exec = require('child_process').execSync;

var re = /\/\/ .*|[ \n]/g;

var source = process.argv[2];
var target = source.replace(".hex", ".bin");

console.log(source + " --> " + target);

var f = fs.readFileSync(source).toString();

var hex = f.replace(re, "");

exec("echo \"" + hex + "\" | xxd -r -p - " + target);
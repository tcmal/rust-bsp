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

extern crate stockton_bsp;

use stockton_bsp::BSPFile;

#[test]
fn test_basic() {
    let data = include_bytes!("./13power.bsp");

    let _lump = BSPFile::from_buffer(data).unwrap();
}

#[test]
fn test_clone() {
    let data = include_bytes!("./13power.bsp");

    let orig = BSPFile::from_buffer(data).unwrap();

    let clone1 = orig.clone();
    let clone2 = orig.clone();

    move || orig;

    assert_eq!(clone1.brushes.brushes[0].texture.name, "textures/gothic_trim/pitted_rust\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}");
    assert_eq!(clone2.brushes.brushes[0].texture.name, "textures/gothic_trim/pitted_rust\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}");
}

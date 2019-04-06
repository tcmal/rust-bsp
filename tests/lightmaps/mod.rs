// Copyright (C) 2019 Oscar Shrimpton
// 
// This file is part of rust_bsp.
// 
// rust_bsp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rust_bsp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rust_bsp.  If not, see <http://www.gnu.org/licenses/>.

use bsp::lumps::LightmapsLump;

#[test]
fn test_lightmaps() {
    let lump = include_bytes!("./test_lightmaps.bin");

    let parsed = LightmapsLump::from_lump(lump).unwrap();

    assert_eq!(parsed.maps.len(), 1);

    for c in &parsed.maps[0].map {
        for x in 0..128 {
            for y in c[x] {
                assert_eq!(*y, x as u8);
            }
        }
    }
}
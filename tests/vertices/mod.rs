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

use bsp::lumps::vertices::{VerticesLump, TexCoord};
use bsp::types::{Vector3, RGBA};

#[test]
fn test_vertices() {
    let lump = include_bytes!("./test_vertices.bin");
    let parsed = VerticesLump::from_lump(lump).unwrap();

    println!("{:?}", parsed);

    assert_eq!(parsed.vertices.len(), 1);
    
    assert_eq!(parsed.vertices[0].position, Vector3::zero());
    assert_eq!(parsed.vertices[0].tex, TexCoord { u: [1.0, 2.0], v: [3.0, 4.0] });
    assert_eq!(parsed.vertices[0].normal, Vector3 { x: 5.0, y: 6.0, z: 7.0 });
    assert_eq!(parsed.vertices[0].color, RGBA { r: 255, g: 255, b: 255, a: 255 });
}
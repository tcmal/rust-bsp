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

use stockton_bsp::lumps::vertices::{VerticesLump, MeshVertsLump, TexCoord};
use stockton_bsp::types::{RGBA};
use na::Vector3;

#[test]
fn test_vertices() {
    let lump = include_bytes!("./test_vertices.bin");
    let parsed = VerticesLump::from_lump(lump).unwrap();

    assert_eq!(parsed.vertices.len(), 1);
    
    assert_eq!(parsed.vertices[0].position, Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(parsed.vertices[0].tex, TexCoord { u: [1.0, 2.0], v: [3.0, 4.0] });
    assert_eq!(parsed.vertices[0].normal, Vector3::new(5.0, 6.0, 7.0 ));
    assert_eq!(parsed.vertices[0].color, RGBA { r: 255, g: 255, b: 255, a: 255 });
}

#[test]
fn test_meshverts() {
    let lump = include_bytes!("./test_meshvertices.bin");

    let parsed = MeshVertsLump::from_lump(lump).unwrap();

    assert_eq!(parsed.meshverts.len(), 5);
    assert_eq!(parsed.meshverts[0].offset, 0);
    assert_eq!(parsed.meshverts[1].offset, 1);
    assert_eq!(parsed.meshverts[2].offset, 2);
    assert_eq!(parsed.meshverts[3].offset, 3);
    assert_eq!(parsed.meshverts[4].offset, 4);
}
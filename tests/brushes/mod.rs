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

use bsp::lumps::brushes::BrushesLump;
use bsp::lumps::textures::{ContentsFlags, SurfaceFlags, Texture, TexturesLump};

#[test]
fn test_brushes() {
    let buf = include_bytes!("./test_brushes.bin");

    let brushes = &buf[..0x24];
    let sides = &buf[0x24..];

    let textures = TexturesLump {
        textures: vec![
            Texture {
                name: "One",
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
            Texture {
                name: "Two",
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
            Texture {
                name: "Three",
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
        ]
        .into_boxed_slice(),
    };

    let parsed = BrushesLump::from_lump(brushes, sides, &textures).unwrap();

    assert_eq!(*parsed.brushes[0].texture, textures.textures[0]);
    assert_eq!(parsed.brushes[0].sides.len(), 1);

    assert_eq!(*parsed.brushes[1].texture, textures.textures[1]);
    assert_eq!(parsed.brushes[1].sides.len(), 3);

    assert_eq!(*parsed.brushes[2].texture, textures.textures[2]);
    assert_eq!(parsed.brushes[2].sides.len(), 0);
}
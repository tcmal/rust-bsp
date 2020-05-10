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

use na::Vector3;
use stockton_bsp::lumps::brushes::BrushesLump;
use stockton_bsp::lumps::planes::{Plane, PlanesLump};
use stockton_bsp::lumps::textures::{ContentsFlags, SurfaceFlags, Texture, TexturesLump};

#[test]
fn test_brushes() {
    let buf = include_bytes!("./test_brushes.bin");

    let brushes = &buf[..0x24];
    let sides = &buf[0x24..];

    let textures = TexturesLump {
        textures: vec![
            Texture {
                name: "One".to_string(),
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
            Texture {
                name: "Two".to_string(),
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
            Texture {
                name: "Three".to_string(),
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            },
        ]
        .into_boxed_slice(),
    };

    let planes = PlanesLump {
        planes: vec![
            Plane {
                normal: Vector3::new(0.0, 0.0, 0.0),
                dist: 1.0,
                complement_normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Plane {
                normal: Vector3::new(0.0, 0.0, 0.0),
                dist: 2.0,
                complement_normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Plane {
                normal: Vector3::new(0.0, 0.0, 0.0),
                dist: 3.0,
                complement_normal: Vector3::new(0.0, 0.0, 0.0),
            },
        ]
        .into_boxed_slice(),
    };

    let parsed = BrushesLump::from_lump(brushes, sides, &textures, &planes).unwrap();

    assert_eq!(parsed.brushes[0].texture_idx, 0);
    assert_eq!(parsed.brushes[0].sides.len(), 1);
    assert_eq!(parsed.brushes[0].sides[0].plane_idx, 0);
    assert_eq!(parsed.brushes[0].sides[0].texture_idx, 0);

    assert_eq!(parsed.brushes[1].texture_idx, 1);
    assert_eq!(parsed.brushes[1].sides.len(), 2);
    assert_eq!(parsed.brushes[1].sides[0].plane_idx, 1);
    assert_eq!(parsed.brushes[1].sides[0].texture_idx, 1);
    assert_eq!(parsed.brushes[1].sides[1].plane_idx, 2);
    assert_eq!(parsed.brushes[1].sides[1].texture_idx, 2);

    assert_eq!(parsed.brushes[2].texture_idx, 2);
    assert_eq!(parsed.brushes[2].sides.len(), 0);
}

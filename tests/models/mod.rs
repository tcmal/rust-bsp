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

use na::{Vector2, Vector3};
use stockton_bsp::lumps::brushes::{Brush, BrushesLump};
use stockton_bsp::lumps::faces::{Face, FaceLump, FaceType};
use stockton_bsp::lumps::ModelsLump;

#[test]
fn test_models() {
    let faces = FaceLump {
        faces: vec![Face {
            face_type: FaceType::Polygon,
            texture_idx: 0,
            effect_idx: None,
            vertices_idx: 0..0,
            lightmap_idx: None,
            meshverts_idx: 0..0,
            map_start: Vector2::new(0, 0),
            map_size: Vector2::new(0, 0),
            map_origin: Vector3::new(0.0, 0.0, 0.0),
            map_vecs: [Vector3::new(0.0, 0.0, 0.0); 2],
            normal: Vector3::new(0.0, 0.0, 0.0),
            size: Vector2::new(0, 0),
        }]
        .into_boxed_slice(),
    };
    let brushes = BrushesLump {
        brushes: vec![Brush {
            sides: vec![].into_boxed_slice(),
            texture_idx: 0,
        }]
        .into_boxed_slice(),
    };

    let data = include_bytes!("./test_models.bin");
    let lump = ModelsLump::from_lump(data, &faces, &brushes).unwrap();

    println!("{:?}", lump);

    assert_eq!(lump.models.len(), 1);

    assert_eq!(lump.models[0].mins, Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(lump.models[0].maxs, Vector3::new(4.0, 5.0, 6.0));

    assert_eq!(lump.models[0].faces_idx, 0..1);
    assert_eq!(lump.models[0].brushes_idx, 0..1);
}

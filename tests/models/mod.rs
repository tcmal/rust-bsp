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

use bsp::lumps::faces::{FaceLump, Face, FaceType};
use bsp::lumps::brushes::{BrushesLump, Brush};
use bsp::lumps::textures::{Texture, SurfaceFlags, ContentsFlags};
use bsp::types::{IVector2, Vector3};
use bsp::lumps::ModelsLump;

#[test]
fn test_models() {
    let tex = Texture {
        name: "test",
        surface: SurfaceFlags::SKIP,
        contents: ContentsFlags::SOLID
    };
    let faces = FaceLump {
        faces: vec![
            Face {
                tex: (&tex).into(),
                effect: None,
                face_type: FaceType::Polygon,
                vertices: vec![].into_boxed_slice(),
                meshverts: vec![].into_boxed_slice(),
                lightmap: None,
                map_start: IVector2::zero(),
                map_size: IVector2::zero(),
                map_origin: Vector3::zero(),
                map_vecs: [Vector3::zero(); 2],
                normal: Vector3::zero(),
                size: IVector2::zero()
            }
        ].into_boxed_slice()
    };
    let brushes = BrushesLump {
        brushes: vec![
            Brush {
                sides: vec![].into_boxed_slice(),
                texture: (&tex).into()
            }
        ].into_boxed_slice()
    };

    let data = include_bytes!("./test_models.bin");
    let lump = ModelsLump::from_lump(data, &faces, &brushes, ).unwrap();

    println!("{:?}", lump);

    assert_eq!(lump.models.len(), 1);

    assert_eq!(lump.models[0].mins, Vector3 { x: 1.0, y: 2.0, z: 3.0 });
    assert_eq!(lump.models[0].maxs, Vector3 { x: 4.0, y: 5.0, z: 6.0 });

    assert_eq!(lump.models[0].faces.len(), 1);
    assert_eq!(*lump.models[0].faces[0], faces.faces[0]);

    assert_eq!(lump.models[0].brushes.len(), 1);
    assert_eq!(*lump.models[0].brushes[0], brushes.brushes[0]);
}
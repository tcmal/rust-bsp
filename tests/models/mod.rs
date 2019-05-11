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

use stockton_bsp::lumps::faces::{FaceLump, Face, FaceType};
use stockton_bsp::lumps::brushes::{BrushesLump, Brush};
use stockton_bsp::lumps::textures::{Texture, SurfaceFlags, ContentsFlags};
use stockton_bsp::lumps::ModelsLump;
use na::{Vector2, Vector3};

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
                map_start: Vector2::new(0, 0),
                map_size: Vector2::new(0, 0),
                map_origin: Vector3::new(0.0, 0.0, 0.0),
                map_vecs: [Vector3::new(0.0, 0.0, 0.0); 2],
                normal: Vector3::new(0.0, 0.0, 0.0),
                size: Vector2::new(0, 0)
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

    assert_eq!(lump.models[0].mins, Vector3::new(1.0, 2.0, 3.0 ));
    assert_eq!(lump.models[0].maxs, Vector3::new(4.0, 5.0, 6.0 ));

    assert_eq!(lump.models[0].faces.len(), 1);
    assert_eq!(*lump.models[0].faces[0], faces.faces[0]);

    assert_eq!(lump.models[0].brushes.len(), 1);
    assert_eq!(*lump.models[0].brushes[0], brushes.brushes[0]);
}
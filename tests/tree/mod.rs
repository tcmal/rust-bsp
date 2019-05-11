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

use stockton_bsp::lumps::BSPTree;
use stockton_bsp::lumps::brushes::{BrushesLump, Brush};
use stockton_bsp::lumps::faces::{FaceLump, Face, FaceType};
use stockton_bsp::lumps::textures::{Texture, SurfaceFlags, ContentsFlags};
use stockton_bsp::types::{IVector2, Vector3};

#[test]
fn test_tree() {
    let buf = include_bytes!("./test_tree.bin");

    let tex = Texture {
        name: "one",
        surface: SurfaceFlags::SKIP,
        contents: ContentsFlags::SOLID
    };
    let faces = FaceLump {
        faces: vec![
            Face {
                tex: (&tex).into(),
                effect: None,
                face_type: FaceType::Mesh,
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

    let nodes = &buf[..0x90];
    let leaves = &buf[0x90..];

    let tree = BSPTree::from_lumps(nodes, leaves, &faces, &brushes).unwrap();

    //            0
    //     1            2
    //  l0    3      l1    l2
    //      l3  l4

    assert!(tree.root.children.is_some());

    let root_children = tree.root.children.unwrap();

    assert!(root_children[0].children.is_some());         // 1

    let children_1 = root_children[0].children.as_ref().unwrap();
    assert!(children_1[0].leaf.is_some()); // l0
    assert!(children_1[1].children.as_ref().is_some()); // 3

    let children_3 = children_1[1].children.as_ref().unwrap();
    assert!(children_3[0].leaf.is_some()); // l3
    assert!(children_3[1].leaf.is_some()); // l4

    assert!(root_children[1].children.as_ref().is_some());         // 2

    let children_2 = root_children[1].children.as_ref().unwrap();
    assert!(children_2[0].leaf.is_some()); // l1
    assert!(children_2[1].leaf.is_some()); // l2
}
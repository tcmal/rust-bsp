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

use stockton_bsp::lumps::textures::{Texture, SurfaceFlags, ContentsFlags};
use stockton_bsp::lumps::brushes::{Brush, BrushesLump};
use stockton_bsp::lumps::effects::EffectsLump;
use stockton_bsp::types::TransparentNonNull;

#[test]
fn test_effects() {
    let brushes = BrushesLump {
        brushes: Box::new([Brush {
            sides: vec![].into_boxed_slice(),
            texture: TransparentNonNull::from(&Texture {
                name: "one",
                surface: SurfaceFlags::SKIP,
                contents: ContentsFlags::SOLID,
            }),
        }]),
    };

    let raw = include_bytes!("./test_effects.bin");

    let parsed = EffectsLump::from_lump(raw, &brushes).unwrap();

    assert_eq!(parsed.effects.len(), 1);

    assert_eq!(parsed.effects[0].name, format!("{:64}", "test"));

    assert_eq!(*parsed.effects[0].brush, brushes.brushes[0]);
}
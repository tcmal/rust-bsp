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

use bsp::lumps::effects::EffectsLump;
use bsp::lumps::brushes::{Brush, BrushSide, BrushesLump};

#[test]
fn test_effects() {
    let brushes = BrushesLump {
        brushes: Box::new([
            Brush {
                sides: Box::new([BrushSide { plane: 0, texture: 0 },
                                 BrushSide { plane: 1, texture: 1 }]),
                texture: 1
            }
        ])
    };

    let raw = include_bytes!("./test_effects.bin");

    let parsed = EffectsLump::from_lump(raw, &brushes).unwrap();

    assert_eq!(parsed.effects.len(), 1);

    assert_eq!(parsed.effects[0].name, format!("{:64}", "test"));
    
    assert_eq!(parsed.effects[0].brush.texture, 1);
    assert_eq!(parsed.effects[0].brush.sides, brushes.brushes[0].sides);
}
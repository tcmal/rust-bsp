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

use std::str;

use super::brushes::BrushesLump;
use super::helpers::slice_to_i32;
use crate::types::Result;

/// The size of one effect definition
const EFFECT_SIZE: usize = 64 + 4 + 4;

/// One effect definition
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    /// The name of the effect - always 64 characters long
    pub name: String,

    /// The brush used for this effect
    pub brush_idx: usize

    // todo: unknown: i32
}

/// Lump containing all effects
/// Found at index 12 in a q3 bsp
#[derive(Debug, Clone)]
pub struct EffectsLump {
    pub effects: Box<[Effect]>,
}

impl EffectsLump {
    /// Parses the given lump and links the brush references to the given `BrushesLump`
    pub fn from_lump(lump: &[u8], brushes: &BrushesLump) -> Result<EffectsLump> {
        if lump.len() % EFFECT_SIZE != 0 {
            return Err(invalid_error!("EffectsLump is incorrectly sized"));
        }
        let length = lump.len() / EFFECT_SIZE;

        let mut effects = Vec::with_capacity(length);
        for n in 0..length {
            let raw = &lump[n * EFFECT_SIZE..(n + 1) * EFFECT_SIZE];

            let brush_idx = slice_to_i32(&raw[64..68]) as usize;
            if brush_idx >= brushes.brushes.len() {
                return Err(invalid_error!("Effect references brush that doesn't exist"));
            }

            effects.push(Effect {
                name: str::from_utf8(&raw[..64])?.to_owned(),
                brush_idx
            });
        }

        Ok(EffectsLump {
            effects: effects.into_boxed_slice(),
        })
    }

    pub fn empty() -> EffectsLump {
        EffectsLump {
            effects: vec![].into_boxed_slice(),
        }
    }
}

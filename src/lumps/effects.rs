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

use std::str;

use crate::types::{Error, Result};
use super::brushes::{BrushesLump, Brush};
use super::helpers::slice_to_i32;

/// The size of one effect definition
const EFFECT_SIZE: usize = 64 + 4 + 4;

/// One effect definition
#[derive(Debug, Clone)]
pub struct Effect<'a> {
    /// The name of the effect - always 64 characters long
    pub name: &'a str,

    /// The brush used for this effect
    pub brush: &'a Brush
    
    // todo: unknown: i32
}

/// Lump containing all effects
/// Found at index 12 in a q3 bsp
#[derive(Debug, Clone)]
pub struct EffectsLump<'a> {
    pub effects: Box<[Effect<'a>]>
}

impl<'a> EffectsLump<'a> {
    /// Parses the given lump and links the brush references to the given `BrushesLump`
    pub fn from_lump(lump: &'a [u8], brushes: &'a BrushesLump) -> Result<'a, EffectsLump<'a>> {
        if lump.len() % EFFECT_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / EFFECT_SIZE;

        let mut effects = Vec::with_capacity(length);

        for n in 0..length {
            let raw = &lump[n*EFFECT_SIZE..(n + 1) * EFFECT_SIZE];
            let brush_id = slice_to_i32(&raw[64..68]) as usize;
            effects.push(Effect {
                name: str::from_utf8(&raw[..64]).unwrap(),
                brush: &brushes.brushes[brush_id]
            });
        }

        Ok(EffectsLump { effects: effects.into_boxed_slice() })
    }
}
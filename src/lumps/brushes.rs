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

//! Parses the brushes & brushsides lumps from a bsp file

/// The size of one brush record. 
const BRUSH_SIZE: usize = (4 * 3);

/// The size of one brushsize record
const SIDE_SIZE: usize = (4 * 2);

use crate::lumps::textures::{Texture, TexturesLump};
use crate::lumps::helpers::slice_to_i32;
use crate::types::{Result, Error, TransparentNonNull};

/// A brushes lump from a bsp file.
/// BrushSides are also stored inside here.
#[derive(Debug, Clone)]
pub struct BrushesLump<'a> {
    pub brushes: Box<[Brush<'a>]>
}

impl<'a> BrushesLump<'a> {
    /// Parse the brushes & brushsides lump from a bsp file.
    pub fn from_lump(brushes_lump: &'a [u8], brush_sides_lump: &'a [u8], textures_lump: &TexturesLump<'a>) -> Result<'static, BrushesLump<'a>> {
        
        if brushes_lump.len() % BRUSH_SIZE != 0 || brush_sides_lump.len() % SIDE_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = brushes_lump.len() / BRUSH_SIZE;
        let mut brushes = Vec::with_capacity(length as usize);

        for n in 0..length {
            let offset = n * BRUSH_SIZE;
            let brush = &brushes_lump[offset..offset + BRUSH_SIZE];
            let texture_index = slice_to_i32(&brush[8..12]) as usize;
            if texture_index >= textures_lump.textures.len() {
                return Err(Error::BadRef { loc: "Brush.Texture", val: texture_index })
            }

            brushes.push(Brush {
                sides: BrushesLump::get_sides(brush_sides_lump, slice_to_i32(&brush[0..4]), slice_to_i32(&brush[4..8])),
                texture: (&textures_lump.textures[texture_index]).into()
            });
        }

        Ok(BrushesLump { brushes: brushes.into_boxed_slice() })
    }

    /// Internal function to get the relevant brushsides for a brush from the data in the brush lump.
    fn get_sides(brush_sides_lump: &[u8], start: i32, length: i32) -> Box<[BrushSide]> {
        let mut sides = Vec::with_capacity(length as usize);

        if length > 0 {
            for n in start..start + length {
                let offset = n as usize * SIDE_SIZE;
                let brush = &brush_sides_lump[offset..offset + SIDE_SIZE];

                sides.push(BrushSide {
                    plane: slice_to_i32(&brush[0..4]),
                    texture: slice_to_i32(&brush[4..8])
                });
            }
        }

        sides.into_boxed_slice()
    }

    /// Helper function to get an empty brushes lump. 
    /// This is used when initialising a BSP file because of references.
    pub fn empty() -> BrushesLump<'static> {
        BrushesLump { brushes: vec![].into_boxed_slice() }
    }
}

/// One brush record. Used for collision detection.
/// "Each brush describes a convex volume as defined by its surrounding surfaces."
#[derive(Debug, Clone, PartialEq)]
pub struct Brush<'a> {
    pub sides: Box<[BrushSide]>,
    pub texture: TransparentNonNull<Texture<'a>>
}

/// Bounding surfacce for brush.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrushSide {
    pub plane: i32,
    pub texture: i32
}
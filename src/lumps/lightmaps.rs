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

//! Parses the lightmaps lump

use std::convert::TryInto;
use std::fmt;

use crate::types::{Result, Error};

/// The size of one lightmap
const LIGHTMAP_SIZE: usize = 128 * 128 * 3;

/// Stores light map textures that help make surface lighting more realistic
#[derive(Clone)]
pub struct Lightmap<'a> {
    pub map: [[&'a [u8]; 128]; 3]
}

impl<'a> fmt::Debug for Lightmap<'a> {
    // rust doesn't implement debug for 3d arrays so done manually
    // \_( )_/
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LightMap {{ map: [")?;
        for c in &self.map {
            write!(f, "[")?;
            for x in c.iter() {
                write!(f, "{:?}, ", x)?;
            }
            write!(f, "], ")?;
        }
        write!(f, "}}")
    }
}

/// Stores all the lightmaps parsed from a BSP file.
#[derive(Debug, Clone)]
pub struct LightmapsLump<'a> {
    pub maps: Box<[Lightmap<'a>]>
}

impl<'a> LightmapsLump<'a> {
    /// Parse the lightmap lump from a bsp file.
    pub fn from_lump(lump: &'a [u8]) -> Result<LightmapsLump<'a>> {
        if lump.len() % LIGHTMAP_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / LIGHTMAP_SIZE;
        let mut maps = Vec::with_capacity(length as usize);

        for n in 0..length {
            let raw = &lump[n * LIGHTMAP_SIZE..(n+1) * LIGHTMAP_SIZE];
            let mut map: [[&'a [u8]; 128]; 3] = [[&[0; 128]; 128]; 3];

            for (colour_index, colour) in map.iter_mut().enumerate() {
                for x in 0..128 {
                    let start = (colour_index * 128 * 128) + (x * 128);
                    colour[x] = raw[start..start + 128].try_into().unwrap();
                }
            }
            maps.push(Lightmap {
                map
            })
        }

        Ok(LightmapsLump { maps: maps.into_boxed_slice() })
    }
}
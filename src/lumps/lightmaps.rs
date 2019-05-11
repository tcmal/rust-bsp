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

//! Parses the lightmaps lump

use std::fmt;

use crate::types::{Error, Result, RGB};

/// The size of one lightmap
const LIGHTMAP_SIZE: usize = 128 * 128 * 3;

/// Stores light map textures that help make surface lighting more realistic
#[derive(Clone)]
pub struct Lightmap {
    pub map: [[RGB; 128]; 128],
}

impl PartialEq for Lightmap {
    fn eq(&self, other: &Lightmap) -> bool {
        for x in 0..128 {
            for y in 0..128 {
                if self.map[x][y] != other.map[x][y] {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Debug for Lightmap {
    // rust doesn't implement debug for 3d arrays so done manually
    // \_( )_/
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LightMap {{ map: [")?;
        for c in self.map.iter() {
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
pub struct LightmapsLump {
    pub maps: Box<[Lightmap]>,
}

impl LightmapsLump {
    /// Parse the lightmap lump from a bsp file.
    pub fn from_lump<'a>(lump: &'a [u8]) -> Result<'a, LightmapsLump> {
        if lump.len() % LIGHTMAP_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / LIGHTMAP_SIZE;
        let mut maps = Vec::with_capacity(length as usize);

        for n in 0..length {
            let raw = &lump[n * LIGHTMAP_SIZE..(n + 1) * LIGHTMAP_SIZE];
            let mut map: [[RGB; 128]; 128] = [[RGB::white(); 128]; 128];

            for x in 0..128 {
                for y in 0..128 {
                    let offset = (x * 128 * 3) + (y * 3);
                    map[x][y] = RGB::from_slice(&raw[offset..offset+3]);
                }
            }
            maps.push(Lightmap { map })
        }

        Ok(LightmapsLump {
            maps: maps.into_boxed_slice(),
        })
    }
}
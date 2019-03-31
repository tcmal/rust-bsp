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
#![feature(try_trait)]

#[macro_use]
extern crate bitflags;

pub mod lumps;
pub mod directory;
pub mod types;

use std::option::NoneError;
use std::convert::From;

use directory::Header;
use lumps::{EntitiesLump, PlanesLump, TexturesLump};

#[derive(Debug)]
/// An error encountered while parsing.
pub enum Error<'a> {
    BadMagic {
        expected: &'static [u8],
        actual: &'a [u8]
    },
    BadSize {
        req: u32
    },
    BadFormat,
    Unsupported { version: u32 }
}

impl<'a> From<NoneError> for Error<'a> {
    fn from(_: NoneError) -> Error<'a> {
        Error::BadFormat
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;


/// Represents a parsed BSP file.
#[derive(Debug, Clone)]
pub struct BSPFile<'a> {
    pub directory: Header,
    pub entities: EntitiesLump<'a>,
    pub textures: TexturesLump<'a>,
    pub planes: PlanesLump
}

impl<'a> BSPFile<'a> {
    /// Try to parse the given buffer a a BSP file
    pub fn from_buffer(buf: &'a [u8]) -> Result<BSPFile<'a>> {
        let header = Header::from(buf)?;
        
        match header.version {
            0x2e => {
                // Quake 3
                let entities = EntitiesLump::from_lump(header.get_lump(buf, 0))?;
                let textures = TexturesLump::from_lump(header.get_lump(buf, 1))?;
                let planes = PlanesLump::from_lump(header.get_lump(buf, 2))?;
                Ok(BSPFile {directory: header, entities, textures, planes})
            },
            _ => Err(Error::Unsupported { version: header.version })
        }

    }
}
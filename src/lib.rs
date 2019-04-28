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

pub mod directory;
pub mod lumps;
pub mod types;

use std::pin::Pin;

use directory::Header;
use lumps::{BrushesLump, EntitiesLump, LightVolsLump, PlanesLump, TexturesLump, VerticesLump, MeshVertsLump, LightmapsLump, FaceLump, EffectsLump, BSPTree, VisDataLump};
use types::{Error, Result};

/// Represents a parsed BSP file.
#[derive(Debug, Clone)]
pub struct BSPFile<'a> {
    pub directory: Header,
    pub entities: EntitiesLump<'a>,
    pub textures: TexturesLump<'a>,
    pub planes: PlanesLump,
    pub lightvols: LightVolsLump,
    pub brushes: BrushesLump<'a>,
    pub vertices: VerticesLump,
    pub meshverts: MeshVertsLump,
    pub lightmaps: LightmapsLump,
    pub effects: EffectsLump<'a>,
    pub faces: FaceLump<'a>,
    pub tree: BSPTree<'a>,
    pub visdata: VisDataLump
}

impl<'a> BSPFile<'a> {
    /// Try to parse the given buffer a a BSP file
    pub fn from_buffer(buf: &'a [u8]) -> Result<Pin<Box<BSPFile<'a>>>> {
        let header = Header::from(buf)?;

        match header.version {
            0x2e => {
                // Quake 3

                // Because of the way this works, each "level" is compiled, moved into the struct, then repeat till the whole file is parsed.
                // Each lump can only be parsed once all its dependents are, so the empty function is just a decoy, it should never be exposed.

                // Level 1 - No dependencies
                let entities = EntitiesLump::from_lump(header.get_lump(buf, 0))?;
                let textures = TexturesLump::from_lump(header.get_lump(buf, 1))?;
                let planes = PlanesLump::from_lump(header.get_lump(buf, 2))?;

                let vertices = VerticesLump::from_lump(header.get_lump(buf, 10))?;
                let meshverts = MeshVertsLump::from_lump(header.get_lump(buf, 11))?;

                let lightmaps = LightmapsLump::from_lump(header.get_lump(buf, 14))?;
                let lightvols = LightVolsLump::from_lump(header.get_lump(buf, 15))?;

                let visdata = VisDataLump::from_lump(header.get_lump(buf, 16))?;

                let mut res = Box::pin(BSPFile {
                    directory: header,
                    entities,
                    textures,
                    planes,
                    lightvols,
                    lightmaps,
                    meshverts,
                    vertices,
                    effects: EffectsLump::empty(),
                    brushes: BrushesLump::empty(),
                    faces: FaceLump::empty(),
                    tree: BSPTree::empty(),
                    visdata
                });

                // Then the next level is constructed
                let brushes = BrushesLump::from_lump(header.get_lump(buf, 8), header.get_lump(buf, 9), &res.textures, &res.planes)?;
                // And moved into the *existing* struct
                unsafe {
                    let mut_ref = Pin::as_mut(&mut res);
                    Pin::get_unchecked_mut(mut_ref).brushes = brushes;
                }
                
                // ---
                let effects = EffectsLump::from_lump(header.get_lump(buf, 12), &res.brushes)?;

                unsafe {
                    let mut_ref = Pin::as_mut(&mut res);
                    Pin::get_unchecked_mut(mut_ref).effects = effects;
                }

                // ---
                let faces = FaceLump::from_lump(header.get_lump(buf, 13), &res.textures, &res.effects, &res.vertices, &res.meshverts, &res.lightmaps)?;
                
                unsafe {
                    let mut_ref = Pin::as_mut(&mut res);
                    Pin::get_unchecked_mut(mut_ref).faces = faces;
                }

                // ---
                let tree = BSPTree::from_lumps(header.get_lump(buf, 3), header.get_lump(buf, 4), &res.faces, &res.brushes)?;

                unsafe {
                    let mut_ref = Pin::as_mut(&mut res);
                    Pin::get_unchecked_mut(mut_ref).tree = tree;
                }

                Ok(res)
            }
            _ => Err(Error::Unsupported {
                version: header.version,
            }),
        }

    }
}
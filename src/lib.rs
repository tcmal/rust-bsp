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

#[macro_use]
extern crate bitflags;
extern crate bit_vec;
extern crate nalgebra as na;

#[macro_use]
mod macros;
pub mod directory;
pub mod lumps;
pub mod types;

use lumps::*;
use directory::Header;
use types::{Error, Result};

/// Represents a parsed BSP file.
#[derive(Debug, Clone)]
pub struct BSPFile {
    pub directory: Header,
    pub entities: EntitiesLump,
    pub textures: TexturesLump,
    pub planes: PlanesLump,
    pub light_vols: LightVolsLump,
    pub brushes: BrushesLump,
    pub vertices: VerticesLump,
    pub meshverts: MeshVertsLump,
    pub light_maps: LightMapsLump,
    pub effects: EffectsLump,
    pub faces: FaceLump,
    pub tree: BSPTree,
    pub visdata: VisDataLump,
    pub models: ModelsLump,

    /// Only present for Quake live maps (IBSP47)
    pub advertisements: Option<AdvertisementsLump>
}

impl BSPFile {
    /// Try to parse the given buffer as a BSP file
    pub fn from_buffer(buf: Box<[u8]>) -> Result<BSPFile> {
        let header = Header::from(&buf)?;

        match header.version {
            // Quake 3 or Quake LIVE (IBSP47)
            0x2e | 0x2f => {
                let entities = EntitiesLump::from_lump(header.get_lump(&buf, 0))?;
                let textures = TexturesLump::from_lump(header.get_lump(&buf, 1))?;
                let planes = PlanesLump::from_lump(header.get_lump(&buf, 2))?;
                let vertices = VerticesLump::from_lump(header.get_lump(&buf, 10))?;
                let meshverts = MeshVertsLump::from_lump(header.get_lump(&buf, 11))?;
                let light_maps = LightMapsLump::from_lump(header.get_lump(&buf, 14))?;
                let light_vols = LightVolsLump::from_lump(header.get_lump(&buf, 15))?;
                let visdata = VisDataLump::from_lump(header.get_lump(&buf, 16))?;
                let brushes = BrushesLump::from_lump(
                    header.get_lump(&buf, 8),
                    header.get_lump(&buf, 9),
                    &textures,
                    &planes
                )?;
                let effects = EffectsLump::from_lump(header.get_lump(&buf, 12), &brushes)?;
                let faces = FaceLump::from_lump(
                    header.get_lump(&buf, 13),
                    &textures,
                    &effects,
                    &vertices,
                    &meshverts,
                    &light_maps,
                )?;
                let tree = BSPTree::from_lumps(
                    header.get_lump(&buf, 3),
                    header.get_lump(&buf, 4),
                    header.get_lump(&buf, 5),
                    header.get_lump(&buf, 6),
                    &faces,
                    &brushes,
                )?;

                let models = ModelsLump::from_lump(header.get_lump(&buf, 7), &faces, &brushes)?;

                // Quake Live has an advertisements lump
                let advertisements = if header.version == 0x2f {
                    Some(AdvertisementsLump::from_lump(header.get_lump(&buf, 17))?)
                } else {
                    None
                };

                Ok(BSPFile {
                    directory: header,
                    entities,
                    textures,
                    planes,
                    light_vols,
                    light_maps,
                    vertices,
                    meshverts,
                    visdata,
                    advertisements,
                    brushes,
                    effects,
                    faces,
                    tree,
                    models
                })
            }
            _ => Err(Error::Unsupported {
                version: header.version,
            }),
        }
    }
}

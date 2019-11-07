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

use super::helpers::slice_to_u32;
use crate::types::{Error, Result};

const TEXTURE_LUMP_SIZE: usize = (64 + 4 + 4);

#[derive(Debug, Clone)]
/// Surface descriptions
pub struct TexturesLump<'a> {
    pub textures: Box<[Texture<'a>]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A record from a `TexturesLump`
pub struct Texture<'a> {
    pub name: &'a str,
    pub surface: SurfaceFlags,
    pub contents: ContentsFlags,
}

impl<'a> TexturesLump<'a> {
    /// Try to parse the given buffer as an entities lump.
    /// # Format
    /// Each entity is:
    /// string[64] name     Texture name.
    /// int flags           Surface flags.
    /// int contents        Content flags.
    /// Length of entities is total lump size / TEXTURE_LUMP_SIZE (64 + 4 + 4)
    pub fn from_lump(lump: &'a [u8]) -> Result<TexturesLump<'a>> {
        if lump.is_empty() || lump.len() % TEXTURE_LUMP_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / TEXTURE_LUMP_SIZE;
        let mut textures = Vec::with_capacity(length);

        for n in 0..length {
            let offset = n * TEXTURE_LUMP_SIZE;
            textures.push(Texture {
                name: str::from_utf8(&lump[offset..offset + 64]).unwrap(),
                surface: SurfaceFlags::from_bits(slice_to_u32(&lump[offset + 64..offset + 68]))?,
                contents: ContentsFlags::from_bits(slice_to_u32(&lump[offset + 68..offset + 72]))?,
            });
        }

        Ok(TexturesLump {
            textures: textures.into_boxed_slice(),
        })
    }
}

bitflags!(
    /// Extracted from the Q3 arena engine code.
    /// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/surfaceflags.h
    pub struct SurfaceFlags: u32 {
        /// never give falling damage
        const NO_DAMAGE = 0x1;
        
        /// affects game physics
        const SLICK = 0x2;
        
        /// lighting from environment map
        const SKY = 0x4;

        /// don't make missile explosions        
        const NO_IMPACT = 0x10;

        /// function as a ladder
        const LADDER = 0x8;

        /// don't leave missile marks
        const NO_MARKS = 0x20;
        
        /// make flesh sounds and effects
        const FLESH = 0x40;
        
        /// don't generate a drawsurface at all
        const NODRAW = 0x80;
        
        /// make a primary bsp splitter
        const HINT = 0x01_00;
        
        /// completely ignore, allowing non-closed brushes
        const SKIP = 0x02_00;
        
        /// surface doesn't need a lightmap
        const NO_LIGHT_MAP = 0x04_00;
        
        /// generate lighting info at vertexes
        const POINT_LIGHT = 0x08_00;
        
        /// clanking footsteps
        const METAL_STEPS = 0x10_00;
        
        /// no footstep sounds
        const NO_STEPS = 0x20_00;
        
        /// don't collide against curves with this set
        const NON_SOLID = 0x40_00;
        
        /// act as a light filter during q3map -light
        const LIGHT_FILTER = 0x80_00;
        
        /// do per-pixel light shadow casting in q3map
        const ALPHA_SHADOW = 0x01_00_00;
        
        /// don't dlight even if solid (solid lava, skies)
        const NO_DLIGHT = 0x02_00_00;
        
        /// leave a dust trail when walking on this surface
        const DUST = 0x04_00_00;
    }
);

bitflags!(
    /// Extracted from the Q3 arena engine code. Less documented than `SurfaceFlags`.
    /// https://github.com/id-Software/Quake-III-Arena/blob/master/code/game/surfaceflags.h
    pub struct ContentsFlags: u32 {
        // an eye is never valid in a solid
        const SOLID = 0x1;
        const LAVA = 0x8;
        const SLIME = 0x10;
        const WATER = 0x20;
        const FOG = 0x40;

        const NOT_TEAM1 = 0x00_80;
        const NOT_TEAM2 = 0x01_00;
        const NOT_BOT_CLIP = 0x02_00;

        const AREA_PORTAL = 0x80_00;

        /// bot specific contents type
        const PLAYER_CLIP = 0x01_00_00;

        /// bot specific contents type
        const MONSTER_CLIP = 0x02_00_00;

        const TELEPORTER = 0x04_00_00;
        const JUMP_PAD = 0x08_00_00;
        const CLUSTER_PORTAL = 0x10_00_00;
        const DO_NOT_ENTER = 0x20_00_00;
        const BOT_CLIP = 0x40_00_00;
        const MOVER = 0x80_00_00;
        
        // removed before bsping an entity
        const ORIGIN = 0x01_00_00_00;

        // should never be on a brush, only in game
        const BODY = 0x02_00_00_00;

        /// brush not used for the bsp
        const DETAIL = 0x08_00_00_00;
        
        /// brush not used for the bsp
        const CORPSE = 0x04_00_00_00;

        /// brushes used for the bsp
        const STRUCTURAL = 0x10_00_00_00;
        
        /// don't consume surface fragments inside
        const TRANSLUCENT = 0x20_00_00_00;

        const TRIGGER = 0x40_00_00_00;
        
        /// don't leave bodies or items (death fog, lava)
        const NODROP = 0x80_00_00_00;
    }
);

#[test]
fn textures_single_texture() {
    let buf: &[u8] = &[
        b'T', b'E', b'S', b'T', b' ', b'T', b'E', b'X', b'T', b'U', b'R', b'E', b' ', b' ', b' ',
        b' ', // name (padded to 64 bytes)
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', 0x43, 0x00, 0x04, 0x00, // surface flags
        0x09, 0x00, 0x00, 0x00, // contents flags
    ];

    let lump = TexturesLump::from_lump(buf).unwrap();

    assert_eq!(lump.textures.len(), 1);

    assert_eq!(lump.textures[0].name, format!("{:64}", "TEST TEXTURE"));
    assert_eq!(
        lump.textures[0].surface,
        SurfaceFlags::NO_DAMAGE | SurfaceFlags::SLICK | SurfaceFlags::FLESH | SurfaceFlags::DUST
    );
    assert_eq!(
        lump.textures[0].contents,
        ContentsFlags::SOLID | ContentsFlags::LAVA
    );
}

#[test]
fn textures_multiple_textures() {
    let buf: &[u8] = &[
        b'T', b'E', b'S', b'T', b' ', b'T', b'E', b'X', b'T', b'U', b'R', b'E', b'1', b' ', b' ',
        b' ', // name (padded to 64 bytes)
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', 0x43, 0x00, 0x04, 0x00, // surface flags
        0x09, 0x00, 0x00, 0x00, // contents flags
        b'T', b'E', b'S', b'T', b' ', b'T', b'E', b'X', b'T', b'U', b'R', b'E', b'2', b' ', b' ',
        b' ', // name (padded to 64 bytes)
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', 0x2c, 0x10, 0x00, 0x00, // surface flags
        0x01, 0x00, 0x00, 0x00, // contents flags
        b'T', b'E', b'S', b'T', b' ', b'T', b'E', b'X', b'T', b'U', b'R', b'E', b'3', b' ', b' ',
        b' ', // name (padded to 64 bytes)
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', 0x00, 0x0a, 0x00, 0x00, // surface flags
        0x41, 0x00, 0x00, 0x00, // contents flags
    ];

    let lump = TexturesLump::from_lump(buf).unwrap();

    assert_eq!(lump.textures.len(), 3);

    assert_eq!(lump.textures[0].name, format!("{:64}", "TEST TEXTURE1"));
    assert_eq!(lump.textures[1].name, format!("{:64}", "TEST TEXTURE2"));
    assert_eq!(lump.textures[2].name, format!("{:64}", "TEST TEXTURE3"));

    assert_eq!(
        lump.textures[0].surface,
        SurfaceFlags::NO_DAMAGE | SurfaceFlags::SLICK | SurfaceFlags::FLESH | SurfaceFlags::DUST
    );
    assert_eq!(
        lump.textures[1].surface,
        SurfaceFlags::METAL_STEPS
            | SurfaceFlags::NO_MARKS
            | SurfaceFlags::LADDER
            | SurfaceFlags::SKY
    );
    assert_eq!(
        lump.textures[2].surface,
        SurfaceFlags::POINT_LIGHT | SurfaceFlags::SKIP
    );

    assert_eq!(
        lump.textures[0].contents,
        ContentsFlags::SOLID | ContentsFlags::LAVA
    );
    assert_eq!(lump.textures[1].contents, ContentsFlags::SOLID);
    assert_eq!(
        lump.textures[2].contents,
        ContentsFlags::SOLID | ContentsFlags::FOG
    );
}

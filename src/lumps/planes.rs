// Copyright (C) 2019 Oscar Shrimpton
// 
// This file is part of rust_bsp.
// 
// rust-bsp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rust-bsp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rust-bsp.  If not, see <http://www.gnu.org/licenses/>.

//! Parses the planes lump from a bsp file.

const PLANE_SIZE: usize = (4 * 3) + 4;

use std::convert::TryInto;

use crate::types::Vector3;
use crate::{Result, Error};

/// The planes lump from a BSP file.
/// Found at lump index 2 in a q3 bsp.
#[derive(Debug, Clone)]
pub struct PlanesLump {
    pub planes: Box<[Plane]>
}

impl PlanesLump {
    /// Parse a lump of planes.
    /// A lump is (lump length / plane size) planes long
    pub fn from_lump(lump: &[u8]) -> Result<PlanesLump> {
        let length = lump.len() / PLANE_SIZE;
        
        if lump.is_empty() || lump.len() % PLANE_SIZE != 0 || length % 2 != 0 {
            return Err(Error::BadFormat);
        }

        let mut planes = vec![
            Plane { normal: Vector3::zero(), dist: 0.0, complement_normal: Vector3::zero() }; length / 2
        ].into_boxed_slice();

        for n in (0..length).step_by(2) {
            let offset = n * PLANE_SIZE;
            let plane = &lump[offset..offset + (PLANE_SIZE * 2)];
            planes[n] = Plane {
                normal: Vector3::from_bytes(plane[0..12].try_into().unwrap()),
                dist: f32::from_bits(u32::from_le_bytes(
                    plane[12..16].try_into().unwrap()
                )),
                complement_normal: Vector3::from_bytes(plane[16..28].try_into().unwrap())
            }
        }

        Ok(PlanesLump { planes })
    }
}

/// Generic plane, referenced by nodes & brushsizes
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// Plane normal
    pub normal: Vector3,

    /// Distance from origin to plane along normal
    pub dist: f32,
    
    /// Opposing normal from coincident plane
    /// This comes from the next plane in the lump.
    pub complement_normal: Vector3
}

#[test]
fn planes_lump() {
    //                  x                           y                       z                          dist
    let buf: &[u8] = &[0x66, 0xe6, 0xf6, 0x42,   0xd7, 0x63, 0xe4, 0x43,   0x00, 0x00, 0x61, 0x44,   0x00, 0x00, 0xc8, 0x42,
                       0x00, 0x00, 0x61, 0x44,   0xd7, 0x63, 0xe4, 0x43,   0x66, 0xe6, 0xf6, 0x42,   0x00, 0x00, 0x00, 0x00];

    let lump = PlanesLump::from_lump(buf).unwrap();

    assert_eq!(lump.planes.len(), 1);

    assert_eq!(lump.planes[0].normal.x, 123.45);
    assert_eq!(lump.planes[0].normal.y, 456.78);
    assert_eq!(lump.planes[0].normal.z, 900.00);

    assert_eq!(lump.planes[0].dist, 100.0);

    assert_eq!(lump.planes[0].complement_normal.x, 900.0);
    assert_eq!(lump.planes[0].complement_normal.y, 456.78);
    assert_eq!(lump.planes[0].complement_normal.z, 123.45);

}
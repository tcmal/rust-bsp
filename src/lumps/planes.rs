// Copyright (C) 2019 Oscar Shrimpton
//
// This file is part of stockton-bsp.
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

use super::helpers::{slice_to_f32, slice_to_vec3};
use crate::types::Result;

use na::Vector3;

/// The planes lump from a BSP file.
/// Found at lump index 2 in a q3 bsp.
#[derive(Debug, Clone)]
pub struct PlanesLump {
    pub planes: Box<[Plane]>,
}

impl PlanesLump {
    /// Parse a lump of planes.
    /// A lump is (lump length / plane size) planes long
    pub fn from_lump(lump: &[u8]) -> Result<PlanesLump> {

        let length = lump.len() / PLANE_SIZE;
        if lump.is_empty() || lump.len() % PLANE_SIZE != 0 || length % 2 != 0 {
            return Err(invalid_error!("PlanesLump incorrectly sized"));
        }


        let mut planes = Vec::with_capacity(length / 2);
        for n in 0..length {
            let offset = n * PLANE_SIZE;
            let plane = &lump[offset..offset + PLANE_SIZE];
            planes.push(Plane {
                normal: slice_to_vec3(&plane[0..12]),
                dist: slice_to_f32(&plane[12..16]),
            });
        }

        Ok(PlanesLump {
            planes: planes.into_boxed_slice(),
        })
    }
}

/// Generic plane, referenced by nodes & brushsizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    /// Plane normal
    pub normal: Vector3<f32>,

    /// Distance from origin to plane along normal
    pub dist: f32,
}
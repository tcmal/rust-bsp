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

use super::helpers::{slice_to_u32, slice_to_vec3};
use crate::types::Result;
use na::Vector3;
use std::fmt;

const ADVERTISEMENT_SIZE: usize = 4 + (4 * 3) + (4 * 3 * 4) + 64;

#[derive(Clone)]
pub struct Advertisement {
    pub cell_id: u32,
    pub normal: Vector3<f32>,
    pub rect: [Vector3<f32>; 4],

    /// This is size 64
    pub model: [u8; 64],
}

#[derive(Debug, Clone)]
pub struct AdvertisementsLump {
    pub advertisements: Box<[Advertisement]>,
}

impl AdvertisementsLump {
    pub fn from_lump(buf: &[u8]) -> Result<AdvertisementsLump> {
        if buf.len() % ADVERTISEMENT_SIZE != 0 {
            return Err(invalid_error!("AdvertisementsLump is incorrectly sized"));
        }
        let n_ads = buf.len() / ADVERTISEMENT_SIZE;

        let mut advertisements = Vec::with_capacity(n_ads);
        for n in 0..n_ads {
            let raw = &buf[n * ADVERTISEMENT_SIZE..(n + 1) * ADVERTISEMENT_SIZE];

            // try_into() doesn't work because the array is too big
            let mut model = [0; 64];
            (&mut model).clone_from_slice(&raw[64..128]);

            advertisements.push(Advertisement {
                cell_id: slice_to_u32(&raw[0..4]),
                normal: slice_to_vec3(&raw[4..16]),
                rect: [
                    slice_to_vec3(&raw[16..28]),
                    slice_to_vec3(&raw[28..40]),
                    slice_to_vec3(&raw[40..52]),
                    slice_to_vec3(&raw[52..64]),
                ],
                model
            });
        }

        Ok(AdvertisementsLump {
            advertisements: advertisements.into_boxed_slice(),
        })
    }
}


impl fmt::Debug for Advertisement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Advertisement")
            .field("cell_id", &self.cell_id)
            .field("normal", &self.normal)
            .field("rect", &self.rect)
            .finish()
    }
}
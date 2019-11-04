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
use crate::types::{Result, Error};
use na::Vector3;

const ADVERTISEMENT_SIZE: usize = 4 + (4 * 3) + (4 * 3 * 4) + 64;

#[derive(Debug, Clone)]
pub struct Advertisement<'a> {
	pub cell_d: u32,
	pub normal: Vector3<f32>,
	pub rect: [Vector3<f32>; 4],

	/// This is size 64
	pub model: &'a [u8]
}

#[derive(Debug, Clone)]
pub struct AdvertisementsLump<'a> {
	pub advertisements: Box<[Advertisement<'a>]>
}

impl<'a> AdvertisementsLump<'a> {
	pub fn from_lump(buf: &'a [u8]) -> Result<'a, AdvertisementsLump<'a>> {
		if buf.len() % ADVERTISEMENT_SIZE != 0 {
            return Err(Error::BadFormat);
		}

		let n_ads = buf.len() / ADVERTISEMENT_SIZE;
		let mut advertisements = Vec::with_capacity(2);
		for n in 0..n_ads {
			let raw = &buf[n*ADVERTISEMENT_SIZE..(n+1)*ADVERTISEMENT_SIZE];

			advertisements.push(Advertisement {
				cell_d: slice_to_u32(&raw[0..4]),
				normal: slice_to_vec3(&raw[4..16]),
				rect: [
					slice_to_vec3(&raw[16..28]),
					slice_to_vec3(&raw[28..40]),
					slice_to_vec3(&raw[40..52]),
					slice_to_vec3(&raw[52..64])
				],
				model: &raw[64..128]
			});
		}

		Ok(AdvertisementsLump {
			advertisements: advertisements.into_boxed_slice()
		})
	}
}
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

use std::convert::TryInto;

use crate::types::{Result, Error, RGB};

const VOL_LENGTH: usize = (4 * 3 * 2) + 2;

#[derive(Debug, Clone, Copy)]
pub struct LightVol {
    pub ambient: RGB,
    pub directional: RGB,
    pub dir: [u8; 2]
}

#[derive(Debug, Clone)]
pub struct LightVolsLump {
    pub vols: Box<[LightVol]>
}

impl LightVolsLump {
    pub fn from_lump<'a>(lump: &[u8]) -> Result<'a, LightVolsLump> {
        if lump.len() % VOL_LENGTH != 0 {
            return Err(Error::BadFormat);
        }
        let length = lump.len() / VOL_LENGTH;
        let mut vols = Vec::with_capacity(length);
        for n in 0..length {
            let data = &lump[n * VOL_LENGTH..(n+1) * VOL_LENGTH];
            vols.push(LightVol {
                ambient: RGB::from_slice(&data[0..3]),
                directional: RGB::from_slice(&data[3..6]),
                dir: data[6..8].try_into().unwrap(),
            });
        }
    
        Ok(LightVolsLump {vols: vols.into_boxed_slice()})
    }
}
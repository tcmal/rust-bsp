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

use super::helpers::slice_to_i32;
use crate::types::{Error, Result};

/// Stores cluster-to-cluster visibility information.
#[derive(Debug, Clone)]
pub struct VisDataLump {
    /// Each vector is an array of bools which states if that cluster is visible for this.
    /// For example, if vecs[x][y] == true, then they are visible.
    /// Every slice inside the first slice has the same length.
    pub vecs: Box<[Box<[bool]>]>,
}
impl VisDataLump {
    pub fn from_lump(data: &[u8]) -> Result<VisDataLump> {
        let n_vecs = slice_to_i32(&data[0..4]) as usize;
        let size_vecs = slice_to_i32(&data[4..8]) as usize;

        if data.len() - 8 != (n_vecs * size_vecs) {
            return Err(Error::BadFormat);
        }

        let mut vecs = Vec::with_capacity(n_vecs);
        for n in 0..n_vecs {
            let offset = 8 + (n * size_vecs);
            let slice = &data[offset..offset + size_vecs];

            let mut data = Vec::with_capacity(size_vecs);
            for x in 0..size_vecs {
                if slice[x] == 1 {
                    data.push(true);
                } else if slice[x] == 0 {
                    data.push(false);
                } else {
                    return Err(Error::BadFormat);
                }
            }

            vecs.push(data.into_boxed_slice());
        }

        Ok(VisDataLump {
            vecs: vecs.into_boxed_slice(),
        })
    }
}
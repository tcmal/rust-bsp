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

use super::brushes::BrushesLump;
use super::faces::FaceLump;
use super::helpers::{slice_to_i32, slice_to_vec3};
use crate::types::Result;
use na::Vector3;
use std::ops::Range;

const MODEL_SIZE: usize = (4 * 3 * 2) + (4 * 4);

#[derive(Debug, Clone)]
pub struct Model {
    pub mins: Vector3<f32>,
    pub maxs: Vector3<f32>,
    pub faces_idx: Range<usize>,
    pub brushes_idx: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct ModelsLump {
    pub models: Box<[Model]>,
}

impl ModelsLump {
    pub fn from_lump(
        data: &[u8],
        faces_lump: &FaceLump,
        brushes_lump: &BrushesLump,
    ) -> Result<ModelsLump> {
        if data.len() % MODEL_SIZE != 0 {
            return Err(invalid_error!("ModelsLump is incorrectly sized"));
        }
        let n_models = data.len() / MODEL_SIZE;

        let mut models = Vec::with_capacity(n_models);
        for n in 0..n_models {
            let raw = &data[n * MODEL_SIZE..(n + 1) * MODEL_SIZE];

            let mins = slice_to_vec3(&raw[0..12]);
            let maxs = slice_to_vec3(&raw[12..24]);

            let faces_idx = {
                let start = slice_to_i32(&raw[24..28]) as usize;
                let n = slice_to_i32(&raw[28..32]) as usize;

                if start + n > faces_lump.faces.len() {
                    return Err(invalid_error!("Model references Face that doesn't exist"));
                }

                start..start+n
            };

            let brushes_idx = {
                let start = slice_to_i32(&raw[32..36]) as usize;
                let n = slice_to_i32(&raw[36..40]) as usize;

                if start + n > brushes_lump.brushes.len() {
                    return Err(invalid_error!("Model references Brush that doesn't exist"));
                }

                start..start+n
            };

            models.push(Model {
                mins,
                maxs,
                faces_idx,
                brushes_idx,
            })
        }

        Ok(ModelsLump {
            models: models.into_boxed_slice(),
        })
    }
}

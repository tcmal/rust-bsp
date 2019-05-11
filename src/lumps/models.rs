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

use super::faces::{Face, FaceLump};
use super::brushes::{Brush, BrushesLump};
use crate::types::{Result, Error, TransparentNonNull};
use super::helpers::{slice_to_i32, slice_to_vec3};
use na::Vector3;

const MODEL_SIZE: usize = (4 * 3 * 2) + (4 * 4);

#[derive(Debug, Clone)]
pub struct Model<'a> {
    pub mins: Vector3<f32>,
    pub maxs: Vector3<f32>,
    pub faces: Box<[TransparentNonNull<Face<'a>>]>,
    pub brushes: Box<[TransparentNonNull<Brush<'a>>]>
}

#[derive(Debug, Clone)]
pub struct ModelsLump<'a> {
    pub models: Box<[Model<'a>]>
}

impl<'a> ModelsLump<'a> {
    pub fn from_lump(data: &'a [u8], faces_lump: &FaceLump<'a>, brushes_lump: &BrushesLump<'a>) -> Result<'a, ModelsLump<'a>> {
        if data.len() % MODEL_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let n_models = data.len() / MODEL_SIZE;
        let mut models = Vec::with_capacity(n_models);
        for n in 0..n_models {
            let raw = &data[n * MODEL_SIZE..(n + 1) * MODEL_SIZE];

            let mins = slice_to_vec3(&raw[0..12]);
            let maxs = slice_to_vec3(&raw[12..24]);

            let first_face = slice_to_i32(&raw[24..28]) as usize;
            let n_faces = slice_to_i32(&raw[28..32]) as usize;

            if first_face + n_faces > faces_lump.faces.len() {
                return Err(Error::BadRef { loc: "Model.Face", val: n_faces });
            }

            let faces = (first_face..first_face + n_faces)
                        .map(|x| TransparentNonNull::from(&faces_lump.faces[x]))
                        .collect::<Vec<TransparentNonNull<Face>>>().into_boxed_slice();

            let first_brush = slice_to_i32(&raw[32..36]) as usize;
            let n_brushes = slice_to_i32(&raw[36..40]) as usize;

            if first_brush + n_brushes > brushes_lump.brushes.len() {
                return Err(Error::BadRef { loc: "Model.Brush", val: n_brushes });
            }

            let brushes = (first_brush..first_brush + n_brushes)
                        .map(|x| TransparentNonNull::from(&brushes_lump.brushes[x]))
                        .collect::<Vec<TransparentNonNull<Brush>>>().into_boxed_slice();

            models.push(Model {
                mins, maxs, faces, brushes
            })

        }

        Ok(ModelsLump {
            models: models.into_boxed_slice()
        })
    }

    pub fn empty() -> ModelsLump<'static> {
        ModelsLump {
            models: vec![].into_boxed_slice()
        }
    }
}
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

use super::effects::EffectsLump;
use super::helpers::{slice_to_i32, slice_to_vec2i, slice_to_vec3};
use super::light_maps::LightMapsLump;
use super::textures::TexturesLump;
use super::vertices::{MeshVertsLump, VerticesLump};
use crate::types::Result;
use na::{Vector2, Vector3};

use std::ops::Range;

const FACE_SIZE: usize = (4 * 8) + (4 * 2) + (4 * 2) + (4 * 3) + ((4 * 2) * 3) + (4 * 3) + (4 * 2);

#[derive(Debug, Clone)]
pub struct FaceLump {
    pub faces: Box<[Face]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum FaceType {
    Polygon = 1,
    Patch = 2,
    Mesh = 3,
    Billboard = 4,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub face_type: FaceType,
    pub texture_idx: usize,
    pub effect_idx: Option<usize>,
    pub lightmap_idx: Option<usize>,
    pub vertices_idx: Range<usize>,
    pub meshverts_idx: Range<usize>,

    pub map_start: Vector2<i32>,
    pub map_size: Vector2<i32>,
    pub map_origin: Vector3<f32>,
    pub map_vecs: [Vector3<f32>; 2],
    
    pub normal: Vector3<f32>,
    pub size: Vector2<i32>,
}

impl FaceLump {
    pub fn from_lump(
        data: &[u8],
        textures: &TexturesLump,
        effects: &EffectsLump,
        vertices_lump: &VerticesLump,
        meshverts_lump: &MeshVertsLump,
        light_maps: &LightMapsLump,
    ) -> Result<FaceLump> {
        if data.len() % FACE_SIZE != 0 {
            return Err(invalid_error!("FaceLump is incorrectly sized"));
        }
        let length = data.len() / FACE_SIZE;

        let mut faces = Vec::with_capacity(length);
        for n in 0..length {
            faces.push(Face::from_slice(
                &data[n * FACE_SIZE..(n + 1) * FACE_SIZE],
                textures,
                effects,
                vertices_lump,
                meshverts_lump,
                light_maps,
            )?);
        }

        Ok(FaceLump {
            faces: faces.into_boxed_slice(),
        })
    }
}


impl Face {
    pub fn from_slice(
        data: &[u8],
        textures: &TexturesLump,
        effects: &EffectsLump,
        vertices_lump: &VerticesLump,
        meshverts_lump: &MeshVertsLump,
        lightmaps: &LightMapsLump,
    ) -> Result<Face> {
        if data.len() != FACE_SIZE {
            panic!("tried to call face.from_slice with invalid slice size");
        }

        // texture
        let texture_idx = slice_to_i32(&data[0..4]) as usize;
        if texture_idx >= textures.textures.len() {
            return Err(invalid_error!("Face references Texture that doesn't exist"));
        }

        // effects
        let effect_idx = slice_to_i32(&data[4..8]) as usize;
        let effect_idx = if effect_idx < 0xffffffff {
            if effect_idx >= effects.effects.len() {
                return Err(invalid_error!("Face references Effect that doesn't exist"));
            }

            Some(effect_idx)
        } else {
            None
        };

        // face type
        // TODO
        let face_type: FaceType = unsafe { ::std::mem::transmute(slice_to_i32(&data[8..12])) };

        // vertices
        let vertex_offset = slice_to_i32(&data[12..16]) as usize;
        let vertex_n = slice_to_i32(&data[16..20]) as usize;
        if (vertex_offset + vertex_n) > vertices_lump.vertices.len() {
            return Err(invalid_error!("Face references Vertex that doesn't exist"));
        }

        let vertices_idx = vertex_offset..vertex_offset + vertex_n;

        // meshverts
        let meshverts_offset = slice_to_i32(&data[20..24]) as usize;
        let meshverts_n = slice_to_i32(&data[24..28]) as usize;
        if (meshverts_offset + meshverts_n) > meshverts_lump.meshverts.len() {
            return Err(invalid_error!("Face references MeshVert that doesn't exist"));
        }

        let meshverts_idx = meshverts_offset..meshverts_offset + meshverts_n;

        // lightmap
        let lightmap_idx = slice_to_i32(&data[28..32]) as usize;
        let lightmap_idx = if lightmap_idx < 0xffffffff {
            if lightmap_idx >= lightmaps.maps.len() {
                return Err(invalid_error!("Face references LightMap that doesn't exist"));
            }

            Some(lightmap_idx)
        } else {
            None
        };

        // map properties
        let map_start = slice_to_vec2i(&data[32..40]);
        let map_size = slice_to_vec2i(&data[40..48]);
        let map_origin = slice_to_vec3(&data[48..60]);

        // map_vecs
        let mut map_vecs = [Vector3::new(0.0, 0.0, 0.0); 2];
        for n in 0..2 {
            let offset = 60 + (n * 3 * 4);
            map_vecs[n] = slice_to_vec3(&data[offset..offset + 12]);
        }

        // normal & size
        let normal = slice_to_vec3(&data[84..96]);
        let size = slice_to_vec2i(&data[96..104]);

        Ok(Face {
            face_type,
            texture_idx,
            effect_idx,
            vertices_idx,
            meshverts_idx,
            lightmap_idx,
            map_start,
            map_size,
            map_origin,
            map_vecs,
            normal,
            size,
        })
    }
}
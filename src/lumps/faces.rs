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


use super::effects::{Effect, EffectsLump};
use super::helpers::slice_to_i32;
use super::lightmaps::{Lightmap, LightmapsLump};
use super::textures::{Texture, TexturesLump};
use super::vertices::{MeshVert, MeshVertsLump, Vertex, VerticesLump};
use crate::types::{Error, IVector2, Result, Vector3};
const FACE_SIZE: usize = (4 * 8) + (4 * 2) + (4 * 2) + (4 * 3) + ((4 * 2) * 3) + (4 * 3) + (4 * 2);

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum FaceType {
    Polygon = 1,
    Patch = 2,
    Mesh = 3,
    Billboard = 4,
}

#[derive(Debug, Clone)]
pub struct Face<'a> {
    pub tex: &'a Texture<'a>,
    pub effect: Option<&'a Effect<'a>>,
    pub face_type: FaceType,
    pub vertices: Box<[&'a Vertex]>,
    pub meshverts: Box<[&'a MeshVert]>,
    pub lightmap: Option<&'a Lightmap>,
    pub map_start: IVector2,
    pub map_size: IVector2,
    pub map_origin: Vector3,
    pub map_vecs: [Vector3; 2],
    pub normal: Vector3,
    pub size: IVector2,
}

impl<'a> Face<'a> {
    pub fn from_slice(
        data: &'a [u8],
        textures: &'a TexturesLump,
        effects: &'a EffectsLump,
        vertices_lump: &'a VerticesLump,
        meshverts_lump: &'a MeshVertsLump,
        lightmaps: &'a LightmapsLump,
    ) -> Result<'a, Face<'a>> {
        if data.len() != FACE_SIZE {
            panic!("tried to call face.from_slice with invalid slice size");
        }

        // texture
        let tex_id = slice_to_i32(&data[0..4]) as usize;
        if tex_id >= textures.textures.len() {
            return Err(Error::BadRef {
                loc: "Face.Texture",
                val: tex_id,
            });
        }
        let tex = &textures.textures[tex_id];

        // effects
        let effect_id = slice_to_i32(&data[4..8]) as usize;
        let mut effect = None;
        if effect_id > 0 {
            if effect_id >= effects.effects.len() {
                return Err(Error::BadRef {
                    loc: "Face.Effect",
                    val: effect_id,
                });
            }
            effect = Some(&effects.effects[effect_id]);
        }


        // face type
        let face_type: FaceType = unsafe { ::std::mem::transmute(slice_to_i32(&data[8..12])) };

        // vertices
        let vertex_offset = slice_to_i32(&data[12..16]) as usize;
        let vertex_n = slice_to_i32(&data[16..20]) as usize;

        if (vertex_offset + vertex_n) >= vertices_lump.vertices.len() {
            return Err(Error::BadRef {
                loc: "Face.Vertices",
                val: vertex_offset,
            });
        }

        let mut vertices = Vec::with_capacity(vertex_n);
        for i in vertex_offset..vertex_offset + vertex_n {
            vertices.push(&vertices_lump.vertices[i]);
        }

        let vertices = vertices.into_boxed_slice();

        // meshverts
        let meshverts_offset = slice_to_i32(&data[20..24]) as usize;
        let meshverts_n = slice_to_i32(&data[24..28]) as usize;

        if (meshverts_offset + meshverts_n) >= meshverts_lump.meshverts.len() {
            return Err(Error::BadRef {
                loc: "Face.MeshVerts",
                val: meshverts_offset,
            });
        }

        let mut meshverts = Vec::with_capacity(meshverts_n);
        for i in meshverts_offset..meshverts_offset + meshverts_n {
            meshverts.push(&meshverts_lump.meshverts[i]);
        }

        let meshverts = meshverts.into_boxed_slice();

        // lightmap
        let lightmap_id = slice_to_i32(&data[28..32]) as usize;
        let mut lightmap = None;
        if lightmap_id > 0 {
            if lightmap_id >= lightmaps.maps.len() {
                return Err(Error::BadRef {
                    loc: "Face.Lightmap",
                    val: lightmap_id,
                });
            }
            lightmap = Some(&lightmaps.maps[lightmap_id]);
        }

        // map properties
        let map_start = IVector2::from_slice(&data[32..40]);
        let map_size = IVector2::from_slice(&data[40..48]);
        let map_origin = Vector3::from_slice(&data[48..60]);

        // map_vecs
        let mut map_vecs = [Vector3::zero(); 2];
        for n in 0..2 {
            let offset = 60 + (n * 3 * 4);
            map_vecs[n] = Vector3::from_slice(&data[offset..offset + 12]);
        }

        // normal & size
        let normal = Vector3::from_slice(&data[84..96]);
        let size = IVector2::from_slice(&data[96..104]);

        Ok(Face {
            tex,
            effect,
            face_type,
            vertices,
            meshverts,
            lightmap,
            map_start,
            map_size,
            map_origin,
            map_vecs,
            normal,
            size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FaceLump<'a> {
    pub faces: Box<[Face<'a>]>,
}

impl<'a> FaceLump<'a> {
    pub fn from_lump(
        data: &'a [u8],
        textures: &'a TexturesLump,
        effects: &'a EffectsLump,
        vertices_lump: &'a VerticesLump,
        meshverts_lump: &'a MeshVertsLump,
        lightmaps: &'a LightmapsLump,
    ) -> Result<'a, FaceLump<'a>> {
        if data.len() % FACE_SIZE != 0 {
            return Err(Error::BadFormat);
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
                lightmaps,
            )?);
        }

        Ok(FaceLump {
            faces: faces.into_boxed_slice(),
        })
    }
}
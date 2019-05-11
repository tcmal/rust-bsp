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

use crate::types::{RGBA, Result, Error};
use super::helpers::{slice_to_i32, slice_to_f32, slice_to_vec3};
use na::Vector3;
use std::convert::TryInto;

/// The size of one vertex
const VERTEX_SIZE: usize = (4 * 3) + (2 * 2 * 4) + (4 * 3) + 4;

/// A vertex, used to describe a face.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub tex: TexCoord,
    pub normal: Vector3<f32>,
    pub color: RGBA
}

/// Represents a TexCoord. 0 = surface, 1= lightmap.
/// This could also be written as [[f32; 2]; 2]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexCoord {
    pub u: [f32; 2],
    pub v: [f32; 2]
}

impl TexCoord {
    /// Internal function. Converts a slice to a TexCoord.
    fn from_bytes(bytes: &[u8; 16]) -> TexCoord {
        TexCoord {
            u: [ slice_to_f32(&bytes[0..4]), slice_to_f32(&bytes[4..8]) ],
            v: [ slice_to_f32(&bytes[8..12]), slice_to_f32(&bytes[12..16]) ]
        }
    }
}

/// The Vertices Lump in a BSP file. Stores a list of vertices.
#[derive(Debug, Clone)]
pub struct VerticesLump {
    pub vertices: Box<[Vertex]>
}


impl VerticesLump {
    /// Parse a Vertices Lump from the data in a BSP file.
    pub fn from_lump(lump: &[u8]) -> Result<'static, VerticesLump> {
        
        if lump.len() % VERTEX_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / VERTEX_SIZE;
        let mut vertices = Vec::with_capacity(length as usize);

        for n in 0..length {
            let offset = n * VERTEX_SIZE;
            let vertex = &lump[offset..offset + VERTEX_SIZE];

            vertices.push(Vertex {
                position: slice_to_vec3(&vertex[0..12]),
                tex: TexCoord::from_bytes(&vertex[12..28].try_into().unwrap()),
                normal: slice_to_vec3(&vertex[28..40]),
                color: RGBA::from_slice(&vertex[40..44])
            })
        }

        Ok(VerticesLump { vertices: vertices.into_boxed_slice() })
    }
}

/// A vertex offset, used to describe generalised triangle meshes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeshVert {
    pub offset: i32
}

/// A list of MeshVerts
#[derive(Debug, Clone)]
pub struct MeshVertsLump {
    pub meshverts: Box<[MeshVert]>
}

impl MeshVertsLump {
    /// Parse the given lump as a list of MeshVerts.
    pub fn from_lump(lump: &[u8]) -> Result<'static, MeshVertsLump> {
        
        if lump.len() % 4 != 0 {
            return Err(Error::BadFormat);
        }

        let length = lump.len() / 4;
        let mut meshverts = Vec::with_capacity(length as usize);

        for n in 0..length {
            meshverts.push(MeshVert {
                offset: slice_to_i32(&lump[n * 4..(n + 1) * 4]),
            })
        }

        Ok(MeshVertsLump { meshverts: meshverts.into_boxed_slice() })
    }
}
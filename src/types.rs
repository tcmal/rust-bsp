// Copyright (C) 2019 Oscar Shrimpton
// 
// This file is part of rust_bsp.
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

//! Various types used in parsed BSP files.

use std::convert::TryInto;

/// Generic (x,y,z) struct.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    /// (0, 0, 0)
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Constructs a vector from a byte buffer.
    /// bytes: 12 byte buffer: (x,y,z) as 3 f32s.
    pub fn from_bytes(bytes: [u8; 12]) -> Vector3 {
        Vector3 {
            x: f32::from_bits(u32::from_le_bytes(
                    bytes[0..4].try_into().unwrap()
                )),
            y: f32::from_bits(u32::from_le_bytes(
                    bytes[4..8].try_into().unwrap()
                )),
            z: f32::from_bits(u32::from_le_bytes(
                    bytes[8..12].try_into().unwrap()
                ))
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Vector3 {
        Vector3::from_bytes(bytes.try_into().unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}


impl IVector3 {
    /// (0, 0, 0)
    pub fn zero() -> IVector3 {
        IVector3 { x: 0, y: 0, z: 0 }
    }

    /// Constructs a vector from a byte buffer.
    /// bytes: 12 byte buffer: (x,y,z) as 3 i32s.
    pub fn from_bytes(bytes: [u8; 12]) -> IVector3 {
        IVector3 {
            x: i32::from_le_bytes(
                    bytes[0..4].try_into().unwrap()
                ),
            y: i32::from_le_bytes(
                    bytes[4..8].try_into().unwrap()
                ),
            z: i32::from_le_bytes(
                    bytes[8..12].try_into().unwrap()
                )
        }
    }

    pub fn from_slice(bytes: &[u8]) -> IVector3 {
        IVector3::from_bytes(bytes.try_into().unwrap())
    }
}

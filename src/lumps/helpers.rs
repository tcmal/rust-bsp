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

//! Helper functions for parsing

use std::convert::TryInto;

/// Turn a slice into a le i32, the int datatype in a bsp file.
/// # Panics
/// If slice is not 4 bytes long
pub fn slice_to_i32(slice: &[u8]) -> i32 {
    i32::from_le_bytes(
        slice.try_into().unwrap()
    )
}


/// Turn a slice into a le u32, used for some bitflags.
/// # Panics
/// If slice is not 4 bytes long.
pub fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_le_bytes(
        slice.try_into().unwrap()
    )
}


/// Turn a slice into a le f32, the float datatype in a bsp file.
/// # Panics
/// If slice is not 4 bytes long
pub fn slice_to_f32(slice: &[u8]) -> f32 {
    f32::from_bits(u32::from_le_bytes(
        slice.try_into().unwrap()
    ))
}
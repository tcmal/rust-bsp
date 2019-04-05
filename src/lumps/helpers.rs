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

pub fn slice_to_i32(slice: &[u8]) -> i32 {
    i32::from_le_bytes(
        slice.try_into().unwrap()
    )
}
pub fn slice_to_f32(slice: &[u8]) -> f32 {
    f32::from_bits(u32::from_le_bytes(
        slice.try_into().unwrap()
    ))
}
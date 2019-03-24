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

pub mod quake3;

#[derive(Debug)]
pub enum Error<'a> {
    BadMagic {
        expected: &'static [u8],
        actual: &'a [u8]
    },
    BadSize {
        req: u32
    },
    BadFormat
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

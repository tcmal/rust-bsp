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

use bsp::lumps::VisDataLump;

#[test]
fn test_visdata() {
    let data = include_bytes!("./test_visdata.bin");

    let lump = VisDataLump::from_lump(data).unwrap();

    assert_eq!(lump.vecs.len(), 3);
    assert_eq!(lump.vecs[0].len(), 10);

    // first vec is all true
    for n in 0..10 {
        assert_eq!(lump.vecs[0][n], true);
    }

    // second vec is all false
    for n in 0..10 {
        assert_eq!(lump.vecs[1][n], false);
    }
    // third alternates
    for n in 0..10 {
        assert_eq!(lump.vecs[2][n], n % 2 == 0);
    }
}
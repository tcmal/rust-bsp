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

use stockton_bsp::lumps::VisDataLump;

#[test]
fn test_visdata() {
    let data = include_bytes!("./test_visdata.bin");

    let lump = VisDataLump::from_lump(data).unwrap();

    assert_eq!(lump.vecs.len(), 3);
    assert_eq!(lump.vecs[0].len(), 8);

    // first vec is all true
    assert!(lump.vecs[0].all());

    // second vec is all false
    assert!(lump.vecs[1].none());

    // third alternates
    for n in 0..8 {
        assert_eq!(lump.vecs[2][n], n % 2 == 0);
    }
}

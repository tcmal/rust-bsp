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

use stockton_bsp::lumps::PlanesLump;

#[test]
fn planes_lump() {
    //  x                       y                       z                       dist
    let buf: &[u8] = &[
        0x66, 0xe6, 0xf6, 0x42, 0xd7, 0x63, 0xe4, 0x43, 0x00, 0x00, 0x61, 0x44, 0x00, 0x00, 0xc8, 0x42,
        0x00, 0x00, 0x61, 0x44, 0xd7, 0x63, 0xe4, 0x43, 0x66, 0xe6, 0xf6, 0x42, 0x00, 0x00, 0x00, 0x00
    ];

    let lump = PlanesLump::from_lump(buf).unwrap();

    assert_eq!(lump.planes.len(), 2);

    assert_eq!(lump.planes[0].normal.x, 123.45);
    assert_eq!(lump.planes[0].normal.y, 456.78);
    assert_eq!(lump.planes[0].normal.z, 900.00);

    assert_eq!(lump.planes[0].dist, 100.0);

    assert_eq!(lump.planes[1].normal.x, 900.0);
    assert_eq!(lump.planes[1].normal.y, 456.78);
    assert_eq!(lump.planes[1].normal.z, 123.45);
    assert_eq!(lump.planes[1].dist, 0.0);
}

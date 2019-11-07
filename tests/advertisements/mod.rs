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

use na::Vector3;
use stockton_bsp::lumps::AdvertisementsLump;

#[test]
fn test_advertisements() {
    let data = include_bytes!("./test_advertisements.bin");

    let lump = AdvertisementsLump::from_lump(data).unwrap();

    // every vector is equal to this
    let test_vec: Vector3<f32> = Vector3::new(1.0, 2.0, 3.0);

    assert_eq!(lump.advertisements.len(), 2);

    for ad_idx in 0..2 {
        let ad = &lump.advertisements[ad_idx];
        assert_eq!(ad.cell_id, 1 + ad_idx as u32);

        assert_eq!(ad.normal, test_vec);

        for i in 0..4 {
            assert_eq!(ad.rect[i], test_vec);
        }

        // 1st is all 0x61, 2nd is all 0x62
        assert_eq!(ad.model.len(), 64);
        for i in 0..64 {
            assert_eq!(ad.model[i], 0x61u8 + ad_idx as u8);
        }
    }
}

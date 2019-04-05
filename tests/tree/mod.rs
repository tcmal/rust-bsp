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

use bsp::lumps::BSPTree;

#[test]
fn test_tree() {
    let buf = include_bytes!("./test_tree.bin");

    let nodes = &buf[..0x90];
    let leaves = &buf[0x90..];

    let tree = BSPTree::from_lumps(nodes, leaves).unwrap();

    //            0
    //     1            2
    //  l0    3      l1    l2
    //      l3  l4

    assert!(tree.root.children.is_some());

    let root_children = tree.root.children.unwrap();

    assert!(root_children[0].children.is_some());         // 1

    let children_1 = root_children[0].children.as_ref().unwrap();
    assert!(children_1[0].leaf.is_some()); // l0
    assert!(children_1[1].children.as_ref().is_some()); // 3

    let children_3 = children_1[1].children.as_ref().unwrap();
    assert!(children_3[0].leaf.is_some()); // l3
    assert!(children_3[1].leaf.is_some()); // l4

    assert!(root_children[1].children.as_ref().is_some());         // 2

    let children_2 = root_children[1].children.as_ref().unwrap();
    assert!(children_2[0].leaf.is_some()); // l1
    assert!(children_2[1].leaf.is_some()); // l2
}
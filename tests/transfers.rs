// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

extern crate navitia_model;
use navitia_model::test_utils::*;
use navitia_model::transfers;
use navitia_model::transfers::TransfersMode;
use std::path::Path;

#[test]
//                    206m
// sp_1 *--------------------------------* sp_3
//       \                        ______/
//        \                  ____/
//   65m   \           _____/   146m
//          \    _____/
//           \__/
//           sp_2
//
fn test_generates_transfers() {
    test_in_tmp_dir(|path| {
        let input_dir = "fixtures/transfers/input";
        let model = navitia_model::ntfs::read(input_dir).unwrap();
        let rules: Vec<Box<Path>> = vec![];
        let model = transfers::generates_transfers(
            model,
            100.0,
            0.785,
            120,
            rules,
            &TransfersMode::IntraContributor,
            None,
        )
        .unwrap();
        navitia_model::ntfs::write(&model, path).unwrap();
        compare_output_dir_with_expected(
            &path,
            vec!["transfers.txt"],
            "./fixtures/transfers/output",
        );
    });
}

#[test]
fn test_generates_transfers_with_modification_rules() {
    test_in_tmp_dir(|path| {
        let input_dir = "fixtures/transfers/input";
        let model = navitia_model::ntfs::read(input_dir).unwrap();
        let rules = vec![Path::new("./fixtures/transfers/rules.txt").to_path_buf()];
        let model = transfers::generates_transfers(
            model,
            100.0,
            0.785,
            120,
            rules,
            &TransfersMode::IntraContributor,
            None,
        )
        .unwrap();
        navitia_model::ntfs::write(&model, path).unwrap();
        compare_output_dir_with_expected(
            &path,
            vec!["transfers.txt"],
            "./fixtures/transfers/output_rules",
        );
    });
}

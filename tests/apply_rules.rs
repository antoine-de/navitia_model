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
use navitia_model::apply_rules;
use navitia_model::test_utils::*;
use std::path::Path;

#[test]
fn test_apply_complementary_codes() {
    test_in_tmp_dir(|path| {
        let input_dir = "fixtures/minimal_ntfs";
        let rules =
            vec![Path::new("./fixtures/apply_rules/complementary_codes_rules.txt").to_path_buf()];
        let report_path = path.join("report.json");

        let model = navitia_model::ntfs::read(input_dir).unwrap();
        let mut collections = model.into_collections();
        apply_rules::apply_rules(
            &mut collections,
            rules,
            Path::new(&report_path).to_path_buf(),
        )
        .unwrap();
        let model = navitia_model::Model::new(collections).unwrap();
        navitia_model::ntfs::write(&model, path).unwrap();
        compare_output_dir_with_expected(
            &path,
            vec!["object_codes.txt", "report.json"],
            "./fixtures/apply_rules/output",
        );
    });
}

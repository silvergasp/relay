/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<7c222587ae6ca0a1495f7b2225622edf>>
 */

mod parse_with_provider;

use parse_with_provider::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn fragment_with_invalid_defaultvalue_provider() {
    let input = include_str!("parse_with_provider/fixtures/fragment_with_invalid_defaultvalue_provider.graphql");
    let expected = include_str!("parse_with_provider/fixtures/fragment_with_invalid_defaultvalue_provider.expected");
    test_fixture(transform_fixture, "fragment_with_invalid_defaultvalue_provider.graphql", "parse_with_provider/fixtures/fragment_with_invalid_defaultvalue_provider.expected", input, expected);
}

#[test]
fn fragment_with_invalid_type_provider() {
    let input = include_str!("parse_with_provider/fixtures/fragment_with_invalid_type_provider.graphql");
    let expected = include_str!("parse_with_provider/fixtures/fragment_with_invalid_type_provider.expected");
    test_fixture(transform_fixture, "fragment_with_invalid_type_provider.graphql", "parse_with_provider/fixtures/fragment_with_invalid_type_provider.expected", input, expected);
}

#[test]
fn fragment_with_valid_provider() {
    let input = include_str!("parse_with_provider/fixtures/fragment_with_valid_provider.graphql");
    let expected = include_str!("parse_with_provider/fixtures/fragment_with_valid_provider.expected");
    test_fixture(transform_fixture, "fragment_with_valid_provider.graphql", "parse_with_provider/fixtures/fragment_with_valid_provider.expected", input, expected);
}

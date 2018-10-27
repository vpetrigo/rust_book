extern crate awesome;

use awesome::prettify;

mod common;

#[test]
fn test_prettify() {
    common::setup();
    assert_eq!("{123}", prettify::print("123"));
    assert_eq!("{}", prettify::print(""));
}
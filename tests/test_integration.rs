use study_rust;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, study_rust::add_two(2));
}

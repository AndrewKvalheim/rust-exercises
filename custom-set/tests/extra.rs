use custom_set::*;

#[test]
fn new_from_duplicated_elements() {
    assert_eq!(CustomSet::new(&[1, 2, 2, 3]), CustomSet::new(&[1, 2, 3]));
}

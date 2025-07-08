use fx::Put;

#[test]
fn put_identity_for_leaf() {
    let x = 5;
    let y = 10;
    let z = x.put(y);
    assert_eq!(z, 10);
}

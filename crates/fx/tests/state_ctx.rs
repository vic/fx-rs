use fx::state::State;

#[test]
fn ctx() {
    let s = State::new(|s: i32| (s + 1, s));
    let (s2, v) = s.run(1);
    assert_eq!(s2, 2);
    assert_eq!(v, 1);
}

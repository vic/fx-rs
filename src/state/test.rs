use crate::state::{Read, State, Write};

#[test]
fn handle_read() {
    let e = Read::read().map(|v: usize| v.to_string());

    let state = State::<usize>::handler(10);
    let reader = Read::<usize>::handler();

    let v = e.handle_left(reader).and_nil().handle_left(state).eval();

    assert_eq!(v, Some(("10".to_owned(), 10)));
}

#[test]
fn handle_write() {
    let e = Write::write(99);

    let state = State::<usize>::handler(10);
    let writer = Write::<usize>::handler();

    let v = e.handle_left(writer).and_nil().handle_left(state).eval();

    assert_eq!(v, Some(((), 99)));
}

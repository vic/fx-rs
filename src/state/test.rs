use crate::state::State;

type StUsize<'f> = State<'f, usize>;

#[test]
fn test_read() {
    let e = StUsize::read().map(|v: usize| v.to_string());
    let state = StUsize::rc_handler(10);
    let reader = StUsize::reader();
    let v = e.handle(reader).and_nil().handle(state).eval();
    assert_eq!(v, Some(("10".to_owned(), 10)));
}

#[test]
fn test_write() {
    let e = StUsize::write(99);
    let state = StUsize::rc_handler(10);
    let writer = StUsize::writer();
    let v = e.handle(writer).and_nil().handle(state).eval();
    assert_eq!(v, Some(((), 99)));
}

#[test]
fn test_read_write() {
    let e = StUsize::read()
        .map(|v: usize| v.to_string().chars().rev().collect())
        .flat_map(|s: String| StUsize::write(s.parse::<usize>().unwrap_or(99)));

    let reader = StUsize::reader();
    let writer = StUsize::writer();
    let state = StUsize::rc_handler(12);

    let v = e
        .handle(reader.on_left())
        .handle(writer.on_right())
        .and_collapse()
        .and_nil()
        .handle(state)
        .eval();

    assert_eq!(v, Some(((), 21)));
}

use crate::state::RcState;

type RcUsize = RcState<usize>;

#[test]
fn test_read() {
    let e = RcUsize::read().map(|v: usize| v.to_string());
    let state = RcUsize::state_handler(10);
    let reader = RcUsize::read_handler();
    let v = e.handle(reader).and_nil().handle(state).eval();
    assert_eq!(v, Some(("10".to_owned(), 10)));
}

#[test]
fn test_write() {
    let e = RcUsize::write(99);
    let state = RcUsize::state_handler(10);
    let writer = RcUsize::write_handler();
    let v = e.handle(writer).and_nil().handle(state).eval();
    assert_eq!(v, Some(((), 99)));
}

#[test]
fn test_read_write() {
    let e = RcUsize::read()
        .map(|v: usize| v.to_string().chars().rev().collect())
        .flat_map(|s: String| RcUsize::write(s.parse::<usize>().unwrap_or(99)));

    // A handler is a transformation over an effect Fx<A, U> => Fx<B, V>.
    let reader = RcUsize::read_handler(); // Fx<Reader<St>, _> => Fx<St, _>
    let writer = RcUsize::write_handler(); // Fx<Writer<St>, _> => Fx<St, _>
    let state = RcUsize::state_handler(12); // Fx<St, _> => Fx<Nil, (_, S)>

    let v = e // e is: Fx<And<Reader<St>, Writer<St>>, (Unit, usize)>
        .handle(reader.on_left())
        .handle(writer.on_right())
        .and_collapse() // Fx<And<St, St>, _> => Fx<St, _>
        .and_nil() // Fx<St, _> => Fx<And<St, Nil>, _>
        .handle(state)
        .eval();

    assert_eq!(v, Some(((), 21)));
}

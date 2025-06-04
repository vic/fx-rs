use crate::Fx;

use super::{Item, Stream};

#[test]
fn test_stream_empty_fold() {
    let stream = Stream::empty();
    let folded = stream
        .fold(|_acc: usize, _item: String| Fx::pure(Item::Next(999)))
        .provide_left(10)
        .eval();
    assert_eq!(folded, 10)
}

#[test]
fn test_stream_once_fold() {
    let stream = Stream::single("33".to_owned());
    let folded = stream
        .fold(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Item::Done(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, 43)
}

#[test]
fn test_stream_cons_fold_done_early() {
    let stream = Stream::cons("12".to_owned(), Stream::single("33".to_owned()));
    let folded = stream
        .fold(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Item::Done(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, 22)
}

#[test]
fn test_stream_cons_fold() {
    let stream = Stream::cons("12".to_owned(), Stream::single("33".to_owned()));
    let folded = stream
        .fold(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Item::Next(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, 55)
}

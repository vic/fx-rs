use crate::Fx;

use super::{Acc, Stream};

#[test]
fn test_stream_empty_fold() {
    let stream = Stream::empty();
    let folded = stream
        .fold_stream(|_acc: usize, _item: String| Fx::pure(Acc::Next(999)))
        .provide_left(10)
        .eval();
    assert_eq!(folded, Some(10))
}

#[test]
fn test_stream_once_fold() {
    let stream = Stream::once("33".to_owned());
    let folded = stream
        .fold_stream(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Acc::Done(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, Some(43))
}

#[test]
fn test_stream_cons_fold_done_early() {
    let stream = Stream::cons("12".to_owned(), Stream::once("33".to_owned()));
    let folded = stream
        .fold_stream(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Acc::Done(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, Some(22))
}

#[test]
fn test_stream_cons_fold() {
    let stream = Stream::cons("12".to_owned(), Stream::once("33".to_owned()));
    let folded = stream
        .fold_stream(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Acc::Next(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, Some(55))
}

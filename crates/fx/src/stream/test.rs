use super::super::kernel::fx::Fx;
use super::stream::{Item, Stream};
use crate::core::ability::Abilities;
use crate::core::acc::AccAbilityExt;

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
    let stream = Stream::cons("12".to_owned(), Fx::value(Stream::single("33".to_owned())));
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
    let stream = Stream::cons("12".to_owned(), Fx::value(Stream::single("33".to_owned())));
    let folded = stream
        .fold(|acc: usize, item: String| {
            let val = acc + item.parse::<usize>().unwrap_or(666);
            Fx::pure(Item::Next(val))
        })
        .provide_left(10)
        .eval();
    assert_eq!(folded, 55)
}

#[test]
fn fold_ability_outcomes_into_stream() {
    let e = Abilities::request(1)
        .then(Abilities::request(2))
        .then(Fx::value(()));
    let ab = Abilities::new(|u: usize| Fx::value(u * 10));
    let e = e.via(ab.acc_outcome(Stream::Nil));
    let e = e.flat_map(|(s, _)| s.fold(|acc: usize, item: usize| Fx::pure(Item::Next(acc + item))));
    let e = e.contra_map(|n: usize| ((), (n, ())), |n, _| n);
    let v = e.provide(100).eval();
    assert_eq!(v, 130);
}

use crate::core::handler::Handler;
use crate::{
    core::{ability::Abilities, acc::AccAbilityExt},
    kernel::{ability::Ability, fx::Fx},
};

#[test]
fn fold_outcome() {
    type Ab<'f> = Box<dyn Ability<'f, usize, (usize, ()), usize>>;
    let e: Fx<'_, (Ab<'_>, (usize, ())), usize> =
        Abilities::request(11).then(Abilities::request(22));
    let h = Abilities::new(|n: usize| Fx::value(n * 2))
        .acc_outcome_with::<usize, _>(0, |acc, i| Fx::value(acc + i));
    let e = h.handle(e);
    let v = e.eval();
    assert_eq!(v, (66, 44))
}

#[test]
fn option_outcome() {
    type Ab<'f> = Box<dyn Ability<'f, usize, (Option<String>, ()), String>>;
    let e: Fx<'_, (Ab<'_>, (Option<String>, ())), usize> = Abilities::request(12)
        .map(|s: String| s.chars().rev().collect::<String>())
        .map(|s: String| s.parse::<usize>().unwrap());
    let ab = Abilities::new(|n: usize| Fx::value(n.to_string()));
    let h = ab.acc_outcome_default::<usize>();
    let e = h.handle(e);
    let v = e.eval();
    assert_eq!(v, (Some("12".to_owned()), 21))
}

#[test]
fn some_outcome() {
    type Ab<'f> = Box<dyn Ability<'f, usize, (Option<String>, ()), String>>;
    let e: Fx<'_, (Ab<'_>, (Option<String>, ())), usize> = Abilities::request(12)
        .map(|s: String| s.chars().rev().collect::<String>())
        .map(|s: String| s.parse::<usize>().unwrap());
    let base_handler =
        Abilities::new(|n: usize| Fx::value(n.to_string())).acc_outcome_default::<usize>();
    let h = Box::new(move |fx| {
        base_handler
            .handle(fx)
            .map(|(o, u): (Option<String>, usize)| (o.unwrap(), u))
    });
    let e = h.handle(e);
    let v = e.eval();
    assert_eq!(v, ("12".to_owned(), 21))
}

#[test]
fn vec_outcome() {
    let e = Abilities::request(11).then(Abilities::request(22));
    let h = Abilities::new(|n: usize| Fx::value(n * 2)).acc_outcome_default::<usize>();
    let e = h.handle(e);
    let v = e.eval();
    assert_eq!(v, (vec![22, 44], 44))
}

#[test]
fn acc_outcome() {
    type F<'f> = Box<dyn Ability<'f, u8, (), u8>>;
    type Ab<'f> = Box<dyn Ability<'f, F<'f>, (F<'f>, ()), F<'f>>>;
    let x: F = Abilities::boxed(|n| Fx::value(n + 10));
    let y: F = Abilities::boxed(|n| Fx::value(n * 2));
    let e: Fx<'_, (Ab<'_>, (F<'_>, ())), bool> = Abilities::request(x)
        .then(Abilities::request(y))
        .map(|_| true);
    let h = Abilities::new(Fx::value)
        .acc_outcome_with::<bool, _>(Abilities::boxed(Fx::value), |acc, f: F<'_>| {
            Fx::value(Abilities::boxed(move |n| f.apply(acc.apply(n).eval())))
        });
    let (acc, b) = h.handle(e).eval();
    assert_eq!(acc.apply(2).eval(), 24u8);
    assert_eq!(b, true);
}

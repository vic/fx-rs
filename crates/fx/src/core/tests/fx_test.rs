// Unit tests for core/fx.rs functionality
#[cfg(test)]
mod fx_tests {
    use crate::Fx;
    use std::convert::identity;

    // #[test]
    // fn value() {
    //     let e = Fx::value(22);
    //     assert_eq!(e.eval(), 22)
    // }

    // #[test]
    // fn provide() {
    //     let e = Fx::pending(|n: usize| Fx::value(n * 10));
    //     let e = e.provide(12);
    //     assert_eq!(e.eval(), 120);
    // }

    // #[test]
    // fn func() {
    //     let e = Fx::func(|u: usize| u.to_string());
    //     let e = e.provide(12);
    //     assert_eq!(e.eval(), "12".to_owned())
    // }

    // #[test]
    // fn contra_map() {
    //     let e = Fx::pending(|s: String| Fx::value(s.chars().rev().collect::<String>()));
    //     let e = e.adapt(|n: usize| n.to_string(), |_, _, value| Fx::value(value));
    //     let e = e.provide(12);
    //     assert_eq!(e.eval(), "21".to_owned())
    // }

    // #[test]
    // fn chain() {
    //     let e = Fx::pending(|n: usize| Fx::value(n * 10));
    //     let e = e.adapt(identity, |_, _, n: usize| Fx::value(n.to_string()));
    //     let e = e.provide(12);
    //     assert_eq!(e.eval(), "120".to_owned())
    // }

    // #[test]
    // fn adapt() {
    //     let e = Fx::pending(|n: usize| Fx::value(n * 10));
    //     let e = e.adapt(
    //         |s: String| s.len(),
    //         |_, _, n: usize| Fx::value(n.to_string()),
    //     );
    //     let e = e.provide("hello".to_owned());
    //     assert_eq!(e.eval(), "50".to_owned());
    // }
}

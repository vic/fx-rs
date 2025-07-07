use crate::{Ability, Fx};

// Unit tests for core/acc.rs and outcome accumulation
#[cfg(test)]
mod acc_tests {
    use super::*;

    #[test]
    fn fold_outcome() {
        let e = Ability::request(11).then(Ability::request(22));
        let h =
            Ability::new(|n: usize| Fx::value(n * 2)).acc_outcome_with(0, |acc, i| Fx::value(acc + i));
        let e = h.handle(e);
        let v = e.eval();

        assert_eq!(v, (66, 44))
    }

    #[test]
    fn option_outcome() {
        let e = Ability::request(12)
            .map(|s: String| s.chars().rev().collect::<String>())
            .map(|s: String| s.parse::<usize>().unwrap());
        let ab = Ability::new(|n: usize| Fx::value(n.to_string()));
        let h = ab.acc_outcome_default();
        let e = h.handle(e);
        let v = e.eval();

        assert_eq!(v, (Some("12".to_owned()), 21))
    }

    #[test]
    fn some_outcome() {
        let e = Ability::request(12)
            .map(|s: String| s.chars().rev().collect::<String>())
            .map(|s: String| s.parse::<usize>().unwrap());
        let h = Ability::new(|n: usize| Fx::value(n.to_string()))
            .acc_outcome_default()
            .map(|e| e.map(|(o, u): (Option<String>, usize)| (o.unwrap(), u)));
        let e = h.handle(e);
        let v = e.eval();

        assert_eq!(v, ("12".to_owned(), 21))
    }

    #[test]
    fn vec_outcome() {
        let e = Ability::request(11).then(Ability::request(22));
        let h = Ability::new(|n: usize| Fx::value(n * 2)).acc_outcome_default();
        let e = h.handle(e);
        let v = e.eval();

        assert_eq!(v, (vec![22, 44], 44))
    }

    #[test]
    fn acc_outcome() {
        type F<'f> = Ability<'f, u8, (), u8>;
        let x: F = F::new(|n| Fx::value(n + 10));
        let y: F = F::new(|n| Fx::value(n * 2));
        let e = Ability::request(x).then(Ability::request(y)).map(|_| true);

        let h = Ability::new(Fx::value).acc_outcome_with(2u8, |acc, f: F| f.apply(acc));

        let v = e.via(h).eval();
        assert_eq!(v, (24u8, true))
    }
}

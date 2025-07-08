use crate::core::pair::Pair;

#[test]
fn pair_tuple_fst_snd() {
    let p = (1, "a");
    assert_eq!(p.fst(), 1);
    assert_eq!(p.snd(), "a");
}

#[test]
fn pair_fwd_bwd() {
    let p = (1, 2);
    let q: (i32, i32) = p.fwd();
    assert_eq!(q, (1, 2));
    let r: (i32, i32) = q.bwd();
    assert_eq!(r, (2, 1));
}

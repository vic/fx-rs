use crate::kernel::fx::Fx;

#[test]
fn test_and_nil() {
    let fx = Fx::immediate(21u32, 42u32);
    let fx2 = fx.and_nil::<(u32, ())>();
    let result = fx2.provide((21, ())).eval();
    assert_eq!(result, 42);
}

#[test]
fn test_and_collapse() {
    let fx = Fx::immediate((10u32, 10u32), 20u32);
    let fx2 = fx.and_collapse::<u32>();
    let result = fx2.provide(10).eval();
    assert_eq!(result, 20);
}

#[test]
fn test_and_collapse_left() {
    let fx = Fx::immediate(((3u8, 'a'), 3u8), 100u8);
    let fx2 = fx.and_collapse_left::<u8, char, (u8, char)>();
    let result = fx2.provide((3, 'a')).eval();
    assert_eq!(result, 100);
}

#[test]
fn test_and_rotate() {
    let fx = Fx::immediate((1u8, (2u16, 3u32)), 99u32);
    let fx2 = fx.and_rotate::<u8, u16, u32, (u16, u32), (u8, u16), (u32, (u8, u16))>();
    let result = fx2.provide((3u32, (1u8, 2u16))).eval();
    assert_eq!(result, 99);
}

#[test]
fn test_and_swap() {
    let fx = Fx::immediate((5u8, 10u16), 15u16);
    let fx2 = fx.and_swap::<u8, u16, (u16, u8)>();
    let result = fx2.provide((10u16, 5u8)).eval();
    assert_eq!(result, 15);
}

#[test]
fn test_and_nest_and_flat() {
    let fx = Fx::immediate((3u8, 7u16), 21u16);
    let nested = fx.and_nest::<u8, u16>();
    let inner = nested.provide(3u8).eval();
    let result = inner.provide(7u16).eval();
    assert_eq!(result, 21);
    let fx = Fx::immediate((3u8, 7u16), 21u16);
    let flat = fx.and_nest::<u8, u16>().and_flat::<(u8, u16)>();
    let result2 = flat.provide((4u8, 6u16)).eval();
    assert_eq!(result2, 21);
}

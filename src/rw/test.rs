use crate::rw::{Read, Write};

#[test]
fn test_read() {
    let e = Read::read()
        .map(|v: &usize| v.to_string())
        .map(|v: String| v + "0")
        .map(|v: String| v.chars().rev().collect::<String>());

    let state = &10;
    let reader = Read::reader(move || &*state);

    let v = e.provide_left(reader).eval();
    assert_eq!(v, Some("001".to_string()));
}

// #[test]
// fn test_write() {
//     let e = Write::write(20);

//     let mut state: usize = 10;

//     // let writer = Write::writer(move |v: usize| state = v);

//     // let v = e.provide_left(writer).eval();
//     // assert_eq!(v, Some(()));
//     assert_eq!(state, 20);
// }

// impl<'f, I, S, O> Ability<'f, I, S, O>
// where
//     I: Clone + 'f,
//     S: Clone + 'f,
//     O: Clone + 'f,
// {
//     pub fn new<F>(f: F) -> Self
//     where
//         F: Fn(I) -> Fx<'f, S, O> + 'f,
//     {
//         Self { f: std::rc::Rc::new(f) }
//     }
//     pub fn apply(&self, i: I) -> Fx<'f, S, O> {
//         (self.f)(i)
//     }

//     pub fn request<P>(i: I) -> Fx<'f, P, O>
//     where
//         I: Clone,
//         P: Pair<Self, S>,
//     {
//         State::get().flat_map(|f: Self| f.apply(i))
//     }

//     pub fn handler<B, V, P>(self) -> Handler<'f, P, B, V, V>
//     where
//         B: Clone,
//         V: Clone,
//         P: Pair<Self, B>,
//     {
//         Handler::new(|e| e.provide_left(self))
//     }

//     pub fn imap<Y, F>(self, imap: F) -> Ability<'f, Y, S, O>
//     where
//         Y: Clone + 'f,
//         F: FnOnce(Y) -> I + Clone + 'f,
//     {
//         Ability::new(|y: Y| self.apply(imap(y)))
//     }

//     pub fn hmap<T, U, H>(self, h: H) -> Ability<'f, I, T, U>
//     where
//         T: Clone + 'f,
//         U: Clone + 'f,
//         H: FnOnce(Fx<'f, S, O>) -> Fx<'f, T, U> + Clone + 'f,
//     {
//         Ability::new(|i: I| h(self.apply(i)))
//     }
// }

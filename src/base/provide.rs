use crate::{Fx, Nil};


impl <'a, A: Copy, V> Fx<'a, A, V>  {
    pub fn provide(self, a: A) -> Fx<'a, Nil, V> { 
        self.and_nil().provide_left(a)
    }
}


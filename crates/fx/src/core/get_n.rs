use crate::core::has_put::Has;
use crate::core::state::State;
use crate::kernel::fx::Fx;

impl<'f, S: Clone> State<'f, S> {
    pub fn get2<A, B>() -> Fx<'f, S, (A, B)>
    where
        S: Has<A> + Has<B> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| Self::get::<B>().map(|b: B| (a, b)))
    }
    pub fn get3<A, B, C>() -> Fx<'f, S, (A, B, C)>
    where
        S: Has<A> + Has<B> + Has<C> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
    {
        Self::get::<A>()
            .map_m(|a: A| Self::get::<B>().map_m(|b: B| Self::get::<C>().map(|c: C| (a, b, c))))
    }
    pub fn get4<A, B, C, D>() -> Fx<'f, S, (A, B, C, D)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| Self::get::<D>().map(|d: D| (a, b, c, d)))
            })
        })
    }
    pub fn get5<A, B, C, D, E>() -> Fx<'f, S, (A, B, C, D, E)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + Has<E> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
        E: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| {
                    Self::get::<D>().map_m(|d: D| Self::get::<E>().map(|e: E| (a, b, c, d, e)))
                })
            })
        })
    }
    pub fn get6<A, B, C, D, E, F>() -> Fx<'f, S, (A, B, C, D, E, F)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + Has<E> + Has<F> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
        E: Clone + 'f,
        F: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| {
                    Self::get::<D>().map_m(|d: D| {
                        Self::get::<E>()
                            .map_m(|e: E| Self::get::<F>().map(|f: F| (a, b, c, d, e, f)))
                    })
                })
            })
        })
    }
    pub fn get7<A, B, C, D, E, F, G>() -> Fx<'f, S, (A, B, C, D, E, F, G)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + Has<E> + Has<F> + Has<G> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
        E: Clone + 'f,
        F: Clone + 'f,
        G: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| {
                    Self::get::<D>().map_m(|d: D| {
                        Self::get::<E>().map_m(|e: E| {
                            Self::get::<F>()
                                .map_m(|f: F| Self::get::<G>().map(|g: G| (a, b, c, d, e, f, g)))
                        })
                    })
                })
            })
        })
    }
    pub fn get8<A, B, C, D, E, F, G, H>() -> Fx<'f, S, (A, B, C, D, E, F, G, H)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + Has<E> + Has<F> + Has<G> + Has<H> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
        E: Clone + 'f,
        F: Clone + 'f,
        G: Clone + 'f,
        H: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| {
                    Self::get::<D>().map_m(|d: D| {
                        Self::get::<E>().map_m(|e: E| {
                            Self::get::<F>().map_m(|f: F| {
                                Self::get::<G>().map_m(|g: G| {
                                    Self::get::<H>().map(|h: H| (a, b, c, d, e, f, g, h))
                                })
                            })
                        })
                    })
                })
            })
        })
    }
    pub fn get9<A, B, C, D, E, F, G, H, I>() -> Fx<'f, S, (A, B, C, D, E, F, G, H, I)>
    where
        S: Has<A> + Has<B> + Has<C> + Has<D> + Has<E> + Has<F> + Has<G> + Has<H> + Has<I> + 'f,
        A: Clone + 'f,
        B: Clone + 'f,
        C: Clone + 'f,
        D: Clone + 'f,
        E: Clone + 'f,
        F: Clone + 'f,
        G: Clone + 'f,
        H: Clone + 'f,
        I: Clone + 'f,
    {
        Self::get::<A>().map_m(|a: A| {
            Self::get::<B>().map_m(|b: B| {
                Self::get::<C>().map_m(|c: C| {
                    Self::get::<D>().map_m(|d: D| {
                        Self::get::<E>().map_m(|e: E| {
                            Self::get::<F>().map_m(|f: F| {
                                Self::get::<G>().map_m(|g: G| {
                                    Self::get::<H>().map_m(|h: H| {
                                        Self::get::<I>().map(|i: I| (a, b, c, d, e, f, g, h, i))
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    }
}

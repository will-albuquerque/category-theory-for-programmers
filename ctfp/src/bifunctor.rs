pub trait Bifunctor<A, B> {
    type Target<S, T>;

    fn bimap<C, D, F, G>(self, f: F, g: G) -> Self::Target<C, D>
    where
        F: Fn(A) -> C,
        G: Fn(B) -> D;
}

impl<A, B> Bifunctor<A, B> for (A, B) {
    type Target<S, T> = (S, T);

    fn bimap<C, D, F, G>(self, f: F, g: G) -> Self::Target<C, D>
    where
        F: Fn(A) -> C,
        G: Fn(B) -> D,
    {
        let (x, y) = self;
        (f(x), g(y))
    }
}

#[cfg(test)]
mod tests {
    use crate::Bifunctor;

    #[test]
    fn test_bimap() {
        let f = |x| x + 1;
        let g = |x| 3 * x;
        let x = (1, 1);

        let x = x.bimap(f, g);
        assert_eq!(x, (2, 3));
    }
}

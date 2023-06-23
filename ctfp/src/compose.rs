pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::identity;

    #[test]
    fn test_compose() {
        let f = |x| x + 1;
        assert_eq!(f(1), 2);
        assert_eq!(compose(f, identity)(1), 2);
        assert_eq!(compose(identity, f)(1), 2);
    }
}

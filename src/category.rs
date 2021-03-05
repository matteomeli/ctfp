/// For every type A, return the value passed as input of type A.
///
/// # Example
/// ```
/// use ctfp::id;
///
/// let x = 1;
/// assert_eq!(id(1), 1);
///
/// let y = "OK";
/// assert_eq!(id(y), "OK");
/// ```
pub fn id<A>(a: A) -> A {
    a
}

/// Give 𝑓 that takes an argument of type 𝐴 and returns
/// a 𝐵 and another function 𝑔 that takes a 𝐵 and returns a 𝐶,
/// Ycompose them by passing the result of 𝑓 to 𝑔, returning
/// a new function that takes an 𝐴 and returns a 𝐶.
///
/// # Example
/// ```
/// use ctfp::{id, compose};
///
/// // Let's first define a trivial incrementer function.
/// fn inc(x: i32) -> i32 {
///   x + 1
/// }
///
/// // and cover our bases by confirming inc works as expected.
/// let x = 1;
/// assert_eq!(inc(x), 2);
///
/// // Since we are composing functions on a given value, the syntax is
/// // compose(A, B)(V). Knowing this, our passing test looks like:
/// assert_eq!(compose(id, inc)(1), 2);
/// assert_eq!(compose(inc, id)(1), 2);
///
/// fn double(x: i32) -> i32 {
///    x * 2
/// }
///
/// let x = 1;
/// assert_eq!(compose(inc, double)(1), 4);
///
/// // Composition is also associative:
/// assert_eq!(compose(compose(inc, double), id)(1), compose(inc, compose(double, id))(1));
/// ```
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_works() {
        let inc = |x| x + 1;
        let double = |x| x * 2;
        assert_eq!(compose(inc, double)(2), 6);
        assert_eq!(compose(double, inc)(2), 5);
    }

    #[test]
    fn compose_respects_identity() {
        let inc = |x| x + 1;
        assert_eq!(compose(inc, id)(0), inc(0));
        assert_eq!(compose(id, inc)(0), inc(0));

        let double = |x| x * 2;
        assert_eq!(compose(double, id)(1), double(1));
        assert_eq!(compose(id, double)(1), double(1));
    }

    #[test]
    fn compose_is_associative() {
        let inc = |x| x + 1;
        let double = |x| x * 2;
        assert_eq!(
            compose(compose(inc, double), id)(1),
            compose(inc, compose(double, id))(1)
        );
    }
}

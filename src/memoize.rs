use std::collections::HashMap;
use std::hash::Hash;

/// Take a function f of argument A -> B and chache its computation the first time its run.
/// For subsequent calls to the function with same argument of type A, return the cached result.
/// # Example
/// ```
/// use ctfp::memoize;
///
/// let mut f = memoize(|x| x * 2);
/// assert_eq!(f(2), 4);
/// assert_eq!(f(2), 4)
/// ```
pub fn memoize<A: Eq + Hash + Copy, B: Copy, F: FnMut(A) -> B>(mut f: F) -> impl FnMut(A) -> B {
    let mut cache = HashMap::new();

    move |x| cache.entry(x).or_insert_with(|| f(x)).clone()
}

#[cfg(test)]
mod test {
    use super::memoize;
    use rand::{random, rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn memoize_works() {
        let mut f = memoize(|x| x * 2);

        assert_eq!(f(2), 4);
        assert_eq!(f(2), 4)
    }

    #[test]
    fn memoize_random() {
        let mut random_mem = memoize(|_| random::<i32>());
        let r0 = random_mem(());
        let r1 = random_mem(());
        let r2 = random_mem(());
        let r3 = random_mem(());
        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
        assert_eq!(r0, r3);
    }

    #[test]
    fn memoize_random_with_seed() {
        let mut random_mem =
            memoize(|seed: <StdRng as SeedableRng>::Seed| StdRng::from_seed(seed).gen::<i32>());
        let r0 = random_mem([1; 32]);
        let r1 = random_mem([1; 32]);
        let r2 = random_mem([3; 32]);
        let r3 = random_mem([3; 32]);
        let r4 = random_mem([5; 32]);
        let r5 = random_mem([7; 32]);
        assert_eq!(r0, r1);
        assert_eq!(r2, r3);
        assert_ne!(r4, r5);
    }
}

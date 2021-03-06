# Category Theory for Programmers Challenges

## Chapter 2: Types and Functions

### 2.1 Define a higher-order function (or a function object) memoize in your favorite language

```rust
use std::collections::HashMap;
use std::hash::Hash;

pub fn memoize<A: Eq + Hash + Copy, B: Copy, F: FnMut(A) -> B>(mut f: F) -> impl FnMut(A) -> B {
    let mut cache = HashMap::new();

    move |x| cache.entry(x).or_insert_with(|| f(x)).clone()
}
```

Source code is [here](../src/memoize.rs).

### 2.2 Try to memoize a function from your standard library that you normally use to produce random numbers. Does it work?

No, it doesn't work. It would always return the same number for the same input, which is not what you want from a random generator.

```rust
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
```

Source code is [here](../src/memoize.rs).

### 2.3 Most random number generators can be initialized with a seed. Implement a function that takes a seed, calls the random number generator with that seed, and returns the result. Memoize that function. Does it work?

It always returns the same number for the same seed.

```rust
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
```

### 2.4 Which of these C++ functions are pure? Try to memoize them and observe what happens when you call them multiple times: memoized and not.

1. factorial as defined is a pure function and can be memoized.
2. std::getchar() is not pure and cannot be memoized.
3. function
```C++
bool f() {
    std::cout << "Hello!" << std::endl;
    return true;
}
```
is not pure and cannot be memoized.
4. function
```C++
int f(int x) {
    static int y = 0;
    y += x;
    return y;
}
```
is not pure as it depends on static y, memoizing it will produce wrong results.

### 2.5 How many different functions are there from Bool to Bool? Can you implement them all?

There are 4 (2 input values * 2 output values).

```rust
fn not(b: bool) -> bool {
    !b
}

fn always(_: bool) -> bool {
    true
}

fn never(_: bool) -> bool {
    false
}

// This is equivalent to id::<bool>
fn same(b: bool) -> bool {
    b
}
```

### 2.6 Draw a picture of a category whose only objects are the types `Void`, `()` (unit), and `Bool`; with arrows corresponding to all possible functions between these types. Label the arrows with the names of the functions.

![A possible category for types Void, () and Bool](/assets/images/diagram_2_6.png)

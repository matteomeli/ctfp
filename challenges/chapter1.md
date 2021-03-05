# Category Theory for Programmers Challenges

## Chapter 1: Category: The Essence of Composition

### 1.1 Implement, as best as you can, the identity function in your favorite language (or the second favorite, if your favorite language happens to be Haskell).

```rust
pub fn id<A>(a: A) -> A {
    a
}
```

Source code is [here](../src/category.rs).

### 1.2 Implement the composition function in your favorite language. It takes two functions as arguments and returns a function that is their composition.

```rust
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
```

Source code is [here](../src/category.rs).

### 1.3 Write a program that tries to test that your composition function respects identity.

```rust
#[test]
fn compose_respects_identity() {
    let inc = |x| x + 1;
    assert_eq!(compose(inc, id)(0), inc(0));
    assert_eq!(compose(id, inc)(0), inc(0));

    let double = |x| x * 2;
    assert_eq!(compose(double, id)(1), double(1));
    assert_eq!(compose(id, double)(1), double(1));
}
```

Source code is [here](../src/category.rs).

### 1.4 Is the world-wide web a category in any sense? Are links morphisms?

A category consists of object and arrows that go between them, arrows compose and their composition is associative and each object has an identity arrow that serves as the unit of composition.

We could try and consider each URL a object and the links referenced in one URL to other URL as morphisms between URL objects. Within such construction, the identity could be defined as reloading the same URL in the browser. But do morphism compose? IfI can reach URL B from URL A and URL C from URL B, I can't necessarily go from URL A to URL C.

### 1.5 Is Facebook a category, with people as objects and friendships as morphisms?

No, because if we assume the morphisms in the category to be represented by friendship relationships, if A is friend of B and B is friend of C, it doesn't imply that A is friend of C.

### 1.6 When is a directed graph a category?

When for each node in the graph there is a edge going from the node back into itself and for each 2 edges connecting 3 nodes in the form `a --- b --- c` there is an edge that connects the first node to the last node of those 3 directly that serves as the composition of the original 2 edges like `a --- c`.



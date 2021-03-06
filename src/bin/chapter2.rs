use ctfp::{self, memoize};
use std::thread;
use std::time::{Duration, Instant};

fn slow_func(x: u64) -> u64 {
    let mut y = 0;
    for _ in 0..10 {
        y = y + x;
        thread::sleep(Duration::from_millis(100));
    }
    return y;
}

fn factorial(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    // Exercise 1
    let mut slof_func_mem = memoize::memoize(slow_func);
    let start = Instant::now();
    let res = slof_func_mem(5);
    let duration = start.elapsed();
    println!("Value of slof_func_mem: {} ({:?})", res, duration);
    let start = Instant::now();
    let res = slof_func_mem(5);
    let duration = start.elapsed();
    println!("Value of slof_func_mem: {} ({:?})", res, duration);

    // Exercise 4
    let mut factorial_mem = memoize::memoize(factorial);
    for _ in 0..10 {
        let start = Instant::now();
        let res = factorial_mem(20);
        let duration = start.elapsed();
        println!("Factorial of 20 (memoized): {} ({:?})", res, duration);
    }
}

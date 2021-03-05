use rand::rngs::StdRng;
use rand::{self, Rng};
use std::thread;
use std::time::{Duration, Instant};

use ctfp::{self, memoize::Memoize};

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
    let mut slof_func_mem = Memoize::new(slow_func);
    let start = Instant::now();
    let res = slof_func_mem.call(5);
    let duration = start.elapsed();
    println!("Value of slow_func: {} ({:?})", res, duration);
    let start = Instant::now();
    let res = slof_func_mem.call(5);
    let duration = start.elapsed();
    println!(
        "Value of memoized slow_func again: {} ({:?})",
        res, duration
    );

    // Exercise 2
    let mut rng = rand::thread_rng();
    let mut gen_u64_mem = Memoize::new(|_| rng.gen::<u64>());
    println!("Random number: {}", gen_u64_mem.call(1));
    println!("Random number again: {}", gen_u64_mem.call(1));
    println!("Another random number: {}", gen_u64_mem.call(2));

    // Exercise 3
    let seed_1 = 0u64;
    let seed_2 = 1u64;
    let mut seed_gen_u64_mem = Memoize::new(|x| {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(x);
        rng.gen::<u64>()
    });
    println!("Seeded random number: {:?}", seed_gen_u64_mem.call(seed_1));
    println!(
        "Seeded random number again: {:?}",
        seed_gen_u64_mem.call(seed_1)
    );
    println!(
        "Another seeded random number: {:?}",
        seed_gen_u64_mem.call(seed_2)
    );

    // Exercise 4
    let mut total_duration = Duration::new(0, 0);
    for _ in 0..10 {
        let start = Instant::now();
        let res = factorial(20);
        let duration = start.elapsed();
        total_duration += duration;
        println!("Factorial of 20 again: {} ({:?})", res, duration);
    }
    println!(
        "Factorial of 20 [avg over 10 executions]: ({:?})",
        total_duration / 10
    );

    let mut total_duration = Duration::new(0, 0);
    let mut factorial_mem = Memoize::new(factorial);
    for _ in 0..10 {
        let start = Instant::now();
        let res = factorial_mem.call(20);
        let duration = start.elapsed();
        total_duration += duration;
        println!("Factorial of 20 (memoized): {} ({:?})", res, duration);
    }
    println!(
        "Factorial of 20 (memoized) [avg over 10 executions]: ({:?})",
        total_duration / 10
    );
}

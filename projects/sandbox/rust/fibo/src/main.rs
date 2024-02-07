use big_int::prelude::*;

// Breaks at n = ~42 due to slowness
fn fibo(n: u64) -> u64 {
    if n == 1 || n == 2 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    let max_n = 200000;
    let mut a: Loose<10> = 1.into();
    let mut b: Loose<10> = 1.into();

    for i in 1..max_n {
        (a, b) = (b.clone(), a + b);
    }

    println!("Fibonacci number #{} is {}", max_n, a);
}

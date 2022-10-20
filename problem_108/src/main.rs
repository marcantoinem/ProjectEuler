use primal::Sieve;

fn main() {
    let sieve = Sieve::new(1_000_000_000);
    let n = min_n_sol_exceed(&sieve, 10_000);
    println!("{}", n);
    println!("{:?}", sieve.factor(n*n));
}

fn num_divisor(sieve: &Sieve, n: usize) -> usize {
    let factors = sieve.factor(n);
    let mut product = 1;
    // p: prime, a: exponent
    for (_, a) in factors.unwrap() {
        product *= a + 1;
    }
    product
}

fn min_n_sol_exceed(sieve: &Sieve, sol: usize) -> usize {
    let mut count = 0;
    let mut n: usize = 0;
    while count < sol {
        n += 116396280;
        count = (num_divisor(sieve, n*n) + 1) / 2;
    }
    n
}
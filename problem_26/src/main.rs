use primal::Sieve;
use rug::{Integer, ops::Pow};

fn ord(n: usize, a: usize) -> u32 {
    let mut k = 1;
    loop {
        if Integer::from(a).pow(k) % n == 1 {
            return k
        }
        k += 1;
    }
}

fn main() {
    let sieve = Sieve::new(1_000);
    let mut biggest = 0;
    for prime in sieve.primes_from(7) {
        let order = ord(prime, 10);
        if order > biggest {
            biggest = prime as u32;
        }
    }
    println!("{}", biggest);
}

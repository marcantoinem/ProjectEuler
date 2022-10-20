use primal::Sieve;

fn prime_sum(sieve: &Sieve, n: usize) -> usize {
    sieve.primes_from(2).take_while(|x| x <= &n).sum()
}

const LEN: usize = 1_000_000;

fn main() {
    let sieve = Sieve::new(10_000_000);
    let mut max = 0;
    for i in 0..LEN/10_000 {
        for j in i..LEN/100 {
            let suite = prime_sum(&sieve, j) - prime_sum(&sieve, i);
            let length = sieve.prime_pi(j) - sieve.prime_pi(i);
            if sieve.is_prime(suite) && length > max && suite < LEN {
                max = length;
                println!("{} {} length {} suite {}", i, j, length, suite);
            }
        }
    }
}

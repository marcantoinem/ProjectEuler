use primal_sieve::Sieve;

fn biggest_prime(n: usize) -> usize {
    let max = (n as f64).sqrt() as usize;
    let sieve = Sieve::new(max);
    for prime in sieve
        .primes_from(1)
        .collect::<Vec<usize>>()
        .into_iter()
        .rev()
    {
        if n % prime == 0 {
            return prime;
        }
    }
    1
}

fn main() {
    let n = 600851475143_usize;
    let biggest_prime = biggest_prime(n);
    println!("{} is the largest prime factor.", biggest_prime);
}

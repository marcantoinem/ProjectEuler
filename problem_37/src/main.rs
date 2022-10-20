use primal::Sieve;

fn truncable(sieve: &Sieve, n: usize) -> bool {
    let nb_digits = (n as f64).log10().floor() as u32 + 1;
    for i in 1..nb_digits {
        let truncated_right = n / 10_usize.pow(i);
        if !sieve.is_prime(truncated_right) {
            return false
        }
        let truncated_left = n % 10_usize.pow(i);
        if !sieve.is_prime(truncated_left) {
            return false
        }
    }
    true
}

fn main() {
    let sieve = Sieve::new(1_000_000);
    let mut sum = 0;
    for prime in sieve.primes_from(10) {
        if truncable(&sieve, prime) {
            sum += prime;
            println!("{prime}");
        }
    }
    println!("The sum is {}", sum);
}

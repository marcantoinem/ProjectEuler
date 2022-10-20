use primal::Sieve;


// https://www.geeksforgeeks.org/generate-all-rotations-of-a-number/
fn rotations(n: usize) -> Vec<usize> {
    let mut vec = vec![];
    let num_digits = (n as f64).log10().ceil() as usize;
    let max_pow = 10_usize.pow((num_digits - 1) as u32);

    let mut n = n;
    for _ in 0..(num_digits - 1) {
        let first_digit = n / max_pow;
        let rotation = (n * 10) + first_digit - (first_digit * max_pow * 10);
        vec.push(rotation);
        n = rotation;
    }
    vec
}

fn rotations_prime(sieve: &Sieve, prime: usize) -> bool {
    let rotations = rotations(prime);
        for rotation in rotations {
            if !sieve.is_prime(rotation) {
                return false
            }
    }
    true
}

fn main() {
    let sieve = Sieve::new(1_000_000);
    // The are 13 primes below 100 that are circular;
    let mut count = 13;
    for prime in sieve.primes_from(100).take_while(|x| x < &1_000_000) {
        count += rotations_prime(&sieve, prime) as usize;
    }
    println!("{}", count)
}

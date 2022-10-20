use primal_sieve::Sieve;

fn main() {
    let n = 10_001 as f64;
    let upper_bound = (n * (n.ln() + n.ln().ln() - 0.955)) as usize;
    let sieve = Sieve::new(upper_bound);
    println!("{}", sieve.nth_prime(n as usize));
}

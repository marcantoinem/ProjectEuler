use primal::Sieve;

fn count_divisor(sieve: &Sieve, n: usize) -> usize {
    let factors = sieve.factor(n).unwrap();
    factors.into_iter().map(|(_, power)| power + 1).product()
}

fn main() {
    let sieve = primal::Sieve::new(100_000);
    for n in 2..100_000 {
        let triangle = n*(n+1)/2;
        let num_divisor = count_divisor(&sieve, triangle);
        if num_divisor > 500 {
            println!("{}", triangle);
        }
    }
}

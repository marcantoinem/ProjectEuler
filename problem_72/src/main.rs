use fraction::{Fraction, ToPrimitive};
use primal::Sieve;

fn totient(sieve: &Sieve, n: usize) -> usize {
    let factors = sieve.factor(n).unwrap();
    let product: Fraction = factors.into_iter().map(|(x, _)| Fraction::from(1) - Fraction::new(1_u64, x as u64)).product::<Fraction>();
    (Fraction::from(n) * product).to_usize().unwrap()
}

fn main() {
    let sieve = Sieve::new(1_000_000);
    let d = 1_000_000;
    let mut sum = 0;
    for i in 2..=d {
        sum += totient(&sieve, i);
    }
    println!("{}", sum);
}

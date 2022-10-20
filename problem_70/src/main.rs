use math_arsenal::digit::Digits;
use primal::Sieve;

fn main() {
    let sieve = Sieve::new(100_000);
    let mut min_ratio = 2.0;
    for a in sieve.primes_from(1009) {
        for b in sieve.primes_from(a).take_while(|b| a*b < 10_000_000) {
            let nb = a * b;
            let totient = (a - 1)*(b - 1);
            let mut nb_digits = Digits::from(nb);
            nb_digits.sort();
            let mut totient_digits = Digits::from(totient);
            totient_digits.sort();
            if totient_digits == nb_digits {
                let ratio = nb as f64 / totient as f64;
                if ratio < min_ratio  {
                    min_ratio = ratio;
                    println!("{}*{}={} totient={}, ratio={}", a, b, nb, totient, ratio);
                }
            }
        }
    }
}
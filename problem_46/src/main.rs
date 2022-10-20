use primal::Sieve;

fn is_decomposable(sieve: &Sieve, n: usize) -> bool {
    let max = (n as f64 / 2.0).sqrt().ceil() as usize;
    for x in 1..max {
        let double_sq = 2 * x.pow(2);
        let rest = n - double_sq;
        if sieve.is_prime(rest) {
            return true
        }
    }
    false
}

fn main() {
    let sieve = Sieve::new(20_000);
    for x in 1..10_000 {
        let odd = 2*x + 1;
        if !sieve.is_prime(odd) && !is_decomposable(&sieve, odd){
            println!("{}", odd);
        }
    }
}

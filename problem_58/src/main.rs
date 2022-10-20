use primal::Sieve;

fn main() {
    let sieve = Sieve::new(1_000_000_000);
    let mut ratio = 1.0;
    let mut n: isize = 1;
    while ratio > 0.1 {
        let mut count = 0;
        n += 1;
        for x in 1..=n {
            for i in 1..4 {
                let y = 4*x.pow(2)+2*(2-i)*x+1;
                if sieve.is_prime(y as usize) {
                    count += 1;
                }
            }
        }
        let total = 4*n + 1;
        ratio = count as f64 / total as f64;
    }
    println!("{}", 2*n+1);
}
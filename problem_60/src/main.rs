use primal::Sieve;

fn length(n: usize) -> usize {
    (n as f64).log10().ceil() as usize
}

fn concat(a: usize, b: usize) -> usize {
    a + b * 10_usize.pow(length(a) as u32)
}

fn is_pair(sieve: &Sieve, a: usize, b: usize) -> bool {
    sieve.is_prime(concat(a, b)) && sieve.is_prime(concat(b, a))
}

fn main() {
    let sieve = Sieve::new(10_000_000_000);
    let small = Sieve::new(30_000);
    let mut smallest_sum = usize::MAX;
    for a in small.primes_from(2) {
        for b in small.primes_from(a) {
            if is_pair(&sieve, a, b) {
                for c in small.primes_from(b) {
                    if is_pair(&sieve, a, c) && is_pair(&sieve, b, c) {
                        for d in small.primes_from(c) {
                            if is_pair(&sieve, a, d) && is_pair(&sieve, b, d) && is_pair(&sieve, c, d) {
                                for e in small.primes_from(d) {
                                    let sum = a + b + c + d + e;
                                    if is_pair(&sieve, a, e) && is_pair(&sieve, b, e) && is_pair(&sieve, c, e) && is_pair(&sieve, d, e) && sum < smallest_sum {
                                        smallest_sum = a + b + c + d + e;
                                        println!("{} {} {} {} {}", a, b, c, d, e);
                                    }
                                }
                            }                            
                        }
                    }
                }
            }
        }
    }
    println!("{}", smallest_sum);
}

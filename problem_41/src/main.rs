use primal::Sieve;

pub fn digits(n: usize) -> Vec<usize> {
    let mut num = n;
    let mut vec = vec![];
    while num != 0 {
        vec.push(num % 10);
        num /= 10;
    }
    vec
}

fn is_pandigital(n: usize) -> bool {
    let mut vec = digits(n);
    vec.sort();
    vec.dedup();
    vec.len() == 7 && vec[0] == 1 && vec[6] == 7
}

fn main() {
    let sieve = Sieve::new(7654321);
    let mut largest = 2143;
    for prime in sieve.primes_from(2143) {
        if is_pandigital(prime) {
            largest = prime;
        }
    }
    println!("{}", largest);
}

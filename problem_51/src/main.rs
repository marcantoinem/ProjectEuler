
// fn digits(num: usize) -> impl Iterator<Item = usize> {
//     num.to_string()
//         .chars()
//         .map(|d| d.to_digit(10).unwrap())
//         .collect::<Vec<_>>()
//         .into_iter()
// }

fn digits(num: usize) -> Vec<usize> {
        num.to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
}

fn positions_of(digits: &Vec<usize>, num: usize) -> Vec<usize> {
    let mut vec = vec![];
    for (i, digit) in digits.iter().rev().enumerate() {
        if digit == &num {
            vec.push(i);
        }
    }
    vec
}

fn main() {
    let upper_bound = 100_000_000;
    let sieve = primal::Sieve::new(upper_bound);
    for prime in sieve.primes_from(100).take_while(|x| x < &(upper_bound/10)) {
        let digits = digits(prime);
        let positions = positions_of(&digits, 0);
        if &positions.len() > &0 {
            let mut next_prime = prime;
            let mut prime_count = 1;
            for _ in 1..10 {
                for position in positions.iter() {
                    next_prime += 10_usize.pow(*position as u32);
                }
                if sieve.is_prime(next_prime) {
                    prime_count += 1;
                }
            }
            if prime_count == 8 {
                println!("{}", prime);
                println!("{}", prime_count);
            }
        }
        let positions = positions_of(&digits, 1);
        if &positions.len() > &0 {
            let mut next_prime = prime;
            let mut prime_count = 1;
            for _ in 1..10 {
                for position in positions.iter() {
                    next_prime += 10_usize.pow(*position as u32);
                }
                if sieve.is_prime(next_prime) {
                    prime_count += 1;
                }
            }
            if prime_count == 8 {
                println!("{}", prime);
                println!("{}", prime_count);
            }
        }
    }
}

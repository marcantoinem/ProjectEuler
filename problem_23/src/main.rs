use primal::Sieve;

// Get the sum of the proper divisors
fn d(sieve: &Sieve, n: usize) -> usize {
    let factors = sieve.factor(n);
    let mut product = 1;
    // p: prime, a: exponent
    for (p, a) in factors.unwrap() {
        if a == 1 {
            product *= p + 1;
        } else {
            product *= (p.pow(a as u32 + 1) - 1) / (p - 1)
        }
    }
    product - n
}

fn list_abundant_numbers(sieve: &Sieve, limit: usize) -> Vec<usize> {
    let mut vec = vec![];
    for n in 12..=limit {
        if d(sieve, n) > n {
            vec.push(n);
        }
    }
    vec
}

fn sum_of_two_abundant_number(abundants: &Vec<usize>, n: usize) -> bool {
    for abundant in abundants {
        if n / 2 + 1 < *abundant {
            return false;
        } else if abundants.contains(&(n - abundant)) {
            return true;
        }
    }
    false
}

fn main() {
    let limit = 20161;
    let sieve = Sieve::new(limit);
    let abundants = list_abundant_numbers(&sieve, limit);
    let mut sum = 0;
    for n in 1..=limit {
        if !sum_of_two_abundant_number(&abundants, n) {
            sum += n;
        }
    }
    println!("{}", sum);
}

fn interesting_num() -> Vec<Vec<(usize, usize)>> {
    let mut vec = vec![];
    let other_prime = [
        17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    ];
    for a in 1..8 {
        for b in 0..=a {
            for c in 0..=b {
                for d in 0..=c {
                    for e in 0..=d {
                        for f in 0..=e {
                            for len in 0..other_prime.len() {
                                let mut interesting_number = vec![(2, 2 * a), (3, 2 * b), (5, 2 * c), (7, 2*d), (11, 2*e), (13, 2*f)];
                                for index in 0..len {
                                    interesting_number.push((other_prime[index], 2));
                                }
                                vec.push(interesting_number);
                            }
                        }
                    }
                }
            }
        }
    }
    vec
}

fn num_divisor(factors: &Vec<(usize, usize)>) -> usize {
    let mut product = 1;
    // p: prime, a: exponent
    for (_, a) in factors {
        product *= a + 1;
    }
    product
}

fn defactorize(vec: &Vec<(usize, usize)>) -> u128 {
    let mut product = 1;
    for (prime, a) in vec {
        product *= (*prime as u128).pow(*a as u32 / 2);
    }
    product
}

fn main() {
    let to_depass = 2 * 1_000_000_000 - 1;
    let mut smallest = u128::MAX;
    for factors in interesting_num() {
        if num_divisor(&factors) > to_depass {
            let n = defactorize(&factors);
            if n < smallest {
                println!("{:?}", factors);
                smallest = n;
            }
        }
    }
    println!("{}", smallest);
}

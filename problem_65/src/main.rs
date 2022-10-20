use fraction::{BigFraction, BigUint};

fn continued_fraction_e(n: usize) -> Vec<usize> {
    let mut vec = vec![2];
    for i in 1..=(n/3) {
        vec.push(1);
        vec.push(2*i);
        vec.push(1);
    }
    if n % 3 == 0 {
        vec.pop();
    } else if n % 3 == 2 {
        vec.push(1);
    }
    vec
}

fn develop(vec: &Vec<usize>) -> BigFraction {
    let i = 1;
    let n = vec.len();
    BigFraction::from(vec[0]) + develop_r(vec, i, n)
}

fn develop_r(vec: &Vec<usize>, i: usize, n: usize) -> BigFraction {
    if i < n {
        return BigFraction::from(1) / (BigFraction::from(vec[i]) + develop_r(vec, i + 1, n))   
    } else {
        return BigFraction::from(0_usize);
    }
}

fn digits(n: BigUint) -> Vec<BigUint> {
    let mut num = n;
    let mut vec = vec![];
    while &num != &BigUint::from(0_usize) {
        vec.push(&num % 10_usize);
        num /= 10_usize;
    }
    vec
}

fn main() {
    let continued_fraction = continued_fraction_e(100);
    let fraction = develop(&continued_fraction);
    let numerator = fraction.numer().unwrap();
    let sum: BigUint = digits(numerator.clone()).iter().sum();
    println!("{}", sum);
}

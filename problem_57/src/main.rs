use fraction::{BigFraction, BigUint, Zero};
use indicatif::{ProgressIterator};

fn square_root(x: usize, depth: usize) -> Vec<usize> {
    let a0: f64 = (x as f64).sqrt();

    let mut m: usize = 0;
    let mut d: usize = 1;
    let mut a: usize = a0.floor() as usize;
    let mut vec = vec![a];
    let mut expansion = vec![a];

    let end: usize = a * 2;
    while a != end {
        m = d * a - m;
        d = (x - m * m) / d;
        a = ((a0 + m as f64) / d as f64).floor() as usize;
        expansion.push(a);
    }
    
    let len = expansion.len() - 1;
    if depth == 1 {
        return vec;
    } else if depth <= len {
        for num in &expansion[1..depth] {
            vec.push(*num);
        }
        return vec;
    } else {
        // depth 10,11
        // len 5
        for _ in 0..(depth/len) {
            for num in &expansion[1..] {
                vec.push(*num);
            }
        }
    
        let rest = depth - (depth / len) * (len);
        if rest == 0 {
            vec.pop();
            return vec
        } else {
            for num in &expansion[1..rest] {
                vec.push(*num);
            }
            return vec
        }
    }   
}

fn develop(vec: &Vec<usize>) -> BigFraction {
    let i = 1;
    let n = vec.len();
    BigFraction::from(vec[0]) + develop_r(vec, i, n)
}

fn develop_r(vec: &Vec<usize>, i: usize, n: usize) -> BigFraction {
    if i < n {
        return BigFraction::from(1) / (BigFraction::from(vec[i]) + develop_r(vec, i + 1, n));
    } else {
        return BigFraction::from(0_usize);
    }
}

fn nb_digit(n: BigUint) -> usize {
    let mut m = n;
    let mut i = 0;
    while m > BigUint::zero() {
        m /= BigUint::from(10_usize);
        i += 1;
    }
    i
}

fn main() {
    let mut sum = 0;
    for i in (1..1_002).progress() {
        let sq = square_root(2, i);
        let fraction = develop(&sq);
        if nb_digit(fraction.numer().unwrap().to_owned()) > nb_digit(fraction.denom().unwrap().to_owned()) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

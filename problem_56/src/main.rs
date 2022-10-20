use num::{BigUint, ToPrimitive};

pub fn sum_digits(n: BigUint) -> usize {
    let mut nb = n;
    let mut sum = 0;
    while nb != BigUint::from(0_usize) {
        sum += (&nb % BigUint::from(10_usize)).to_usize().unwrap();
        nb /= BigUint::from(10_usize);
    }
    sum
}

fn main() {
    let mut max = 0;
    for a in 90..100 {
        for b in 90..100 {
            let n = BigUint::from(a as usize).pow(b);
            let sum = sum_digits(n);
            if sum > max {
                max = sum;
            }
        }
    }
    println!("The best is {}.", max);
}
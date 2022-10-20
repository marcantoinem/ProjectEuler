use num::BigUint;

fn main() {
    let nb = BigUint::from(28433_usize) * BigUint::from(2_usize).pow(7830457) + BigUint::from(1_usize);
    let last_ten_digits = nb % BigUint::from(10_000_000_000_usize);
    println!("{}", last_ten_digits);
}

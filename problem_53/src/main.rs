use math_arsenal::factorial::factorial;
use num::BigUint;

fn combination(n: usize, r: usize) -> BigUint {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    let mut sum  = 0;
    for n in 23..=100 {
        let mut r = 1;
        let mut nb = BigUint::from(1_usize);
        while nb <= BigUint::from(1_000_000_usize) {
            r += 1;
            nb = combination(n,r);
        }
        sum += n + 1 - 2*r;
    }
    println!("{}", sum);
}

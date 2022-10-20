fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        if a==0 {return b};
        b %= a;
    }
    a
}

fn main() {
    let a: usize = 1;
    let b: usize = 2;
    let c: usize = 1;
    let d: usize = 3;
    let mut sum = 0;
    for q in 1..=40_000 {
        let mut i = 1;
        let mut p = (a*q - i) / b;
        if c * q < d * p && gcd(p, q) == 1 {
            sum += 1;
        }
        while p >= 1 {
            i += 2;
            p = (a*q - i) / b;
            if c * q < d * p && gcd(p, q) == 1 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

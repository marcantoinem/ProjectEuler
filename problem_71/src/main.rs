fn main() {
    let a: usize = 3;
    let b: usize = 7;
    // r / s
    let mut best = (0,1);
    for q in 1..1_000_000 {
        let p = (a*q - 1) / b;
        if best.0 * q < p * best.1 {
            best = (p,q);
        }
    }
    println!("{:?}", best)
}

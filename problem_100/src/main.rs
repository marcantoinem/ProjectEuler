use std::time::Instant;

// https://oeis.org/A000129
fn main() {
    let instant = Instant::now();
    let mut b: usize = 15;
    let mut n: usize = 21;
    while n <= 1_000_000_000_000 {
        let b_temp = 3*b+2*n-2;
        n = 4 * b + 3 * n - 3;
        b = b_temp;
    }
    println!("total: {} blue: {} computed in {}s", n, b, instant.elapsed().as_secs_f64())
}

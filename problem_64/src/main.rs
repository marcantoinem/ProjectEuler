fn period(x: usize) -> Vec<usize> {
    let a0: f64 = (x as f64).sqrt();

    let mut m: usize = 0;
    let mut d: usize = 1;
    let mut a: usize = a0.floor() as usize;
    let mut expansion = vec![];

    let end: usize = a * 2;
    while a != end {
        m = d * a - m;
        d = (x - m * m) / d;
        a = ((a0 + m as f64) / d as f64).floor() as usize;
        expansion.push(a);
    }
    expansion
}

fn is_square(n: usize) -> bool {
    (n as f64).sqrt().floor().powf(2.0) as usize == n 
}

fn main() {
    let mut sum = 0;
    for n in 2..=10_000 {
        if !is_square(n) {
            let len = period(n).len();
            if len % 2 == 1 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

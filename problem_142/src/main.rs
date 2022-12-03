use std::time::Instant;

fn is_square(n: usize) -> bool {
    let sqrt = (n as f64).sqrt();
    sqrt.round() == sqrt
}

fn main() {
    let time = Instant::now();
    for n_a in 3.. {
        let a = n_a * n_a;
        let parity = n_a % 2;
        for n_c in 2..n_a {
            let c = n_c * n_c;
            if !is_square(a - c) || n_c % 2 != parity {
                continue;
            }
            for n_e in ((a as f64 - c as f64).sqrt().ceil() as usize)..n_c {
                if n_e % 2 != parity {
                    continue;
                }
                let e = n_e * n_e;
                if is_square(c - e) && is_square(a - e) {
                    println!("{} {} {}", a, c, e);
                    println!("{}", c - e);
                    println!("{}", a - e);
                    println!("{}", a - c);
                    println!("{} {} {}", (c - e + a), (e - c + a), (e + c - a));

                    let sum = (a + c + e) / 2;
                    println!(
                        "The minimal sum is {} computed in {}s",
                        sum,
                        time.elapsed().as_secs_f64()
                    );
                    return;
                }
            }
        }
    }
}

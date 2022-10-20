fn reverse_rec(n: usize, len: u32) -> usize {
    let x: usize = 10;
    if len == 1 {
        return n;
    } else {
        return (n % 10) * x.pow(len - 1) + reverse_rec(n / 10, len - 1);
    }
}

fn reverse(n: usize) -> usize {
    let len = (n as f64).log10().ceil() as u32;
    reverse_rec(n, len)
}

fn is_lychrel(n: usize) -> bool {
    let mut m = n + reverse(n);
    for _ in 0..49 {
        let rev = reverse(m);
        if rev == m {
            return false
        }
        m += rev;
    }
    true
}

fn main() {
    let mut count = 0;
    for n in 10..10_000 {
        if is_lychrel(n) {
            count += 1;
        }
    }
    println!("{}", count);
}

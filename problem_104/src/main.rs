use std::time::Instant;

const GOLDEN_NUMBER: f64 = 1.618033988749895;

// Still not finished will be retaped.

pub fn digits(n: usize) -> Vec<usize> {
    let mut num = n;
    let mut vec = vec![];
    while num != 0 {
        vec.push(num % 10);
        num /= 10;
    }
    vec
}

fn is_pandigital_last_first(first_digits: u128, last_digits: u128) -> bool {
    let ten_less = ((first_digits as f64).log10().floor() as u32).saturating_sub(8);
    let first_digits = first_digits as usize / 10_usize.pow(ten_less);
    return is_pandigital(first_digits) && is_pandigital(last_digits as usize);
}

fn is_pandigital(n: usize) -> bool {
    let mut digits = digits(n);
    digits.sort_unstable();
    return digits == vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
}

fn main() {
    let time = Instant::now();
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    for _ in 0..30 {
        a += b;
        b += a;
    }

    let mut n = b;
    let mut i = 63;
    loop {
        a = (a + b) % 1_000_000_000;
        b = (a + b) % 1_000_000_000;
        // Thanks our dear Gilbert Strang for showing this wonderful approximation.
        n = (n as f64 * GOLDEN_NUMBER) as u128;
        let twenty_less = ((n as f64).log10().floor() as u32).saturating_sub(18);
        n = n / 10u128.pow(twenty_less);
        if is_pandigital_last_first(n, a) {
            println!("{}: {}", i, a);
            break;
        }
        n = (n as f64 * GOLDEN_NUMBER) as u128;
        let twenty_less = ((n as f64).log10().floor() as u32).saturating_sub(18);
        n = n / 10u128.pow(twenty_less);
        if is_pandigital_last_first(n, b) {
            println!("{}: computed in {}s", i + 1, time.elapsed().as_secs_f64());
            break;
        }
        i += 2;
        if i > 330000 {
            break;
        }
    }
}

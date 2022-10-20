fn digits(n: usize) -> Vec<usize> {
    let mut num = n;
    let mut vec = vec![];
    while num != 0 {
        vec.push(num % 10);
        num /= 10;
    }
    vec
}

fn main() {
    let mut max = 918273645;
    for nb in 1..1_000_000 {
        let mut k = 1;
        let mut len = 0;
        let mut result = 0;
        loop {
            let next_term = k * nb;
            len += (next_term as f64).log10().floor() as u32 + 1;
            if len < 9 {
                result += k * nb * 10_usize.pow(9 - len);
            } else if len == 9 {
                result += k * nb * 10_usize.pow(9 - len);
                if result > max {
                    let mut digits = digits(result);
                    digits.sort();
                    digits.dedup();
                    if digits.len() == 9 && digits[0] == 1 {
                        println!("{} {}", nb, result);
                        max = result;
                    }
                }
            } else {
                break
            }
            k += 1;
        }
    }
    println!("max: {}", max);
}

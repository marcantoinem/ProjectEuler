fn main() {
    let mut len = 1;
    for nb in 1..500_000 {
        let digit = (nb as f64).log10().floor() as u32 + 1;
        if len > 10 - digit && len <= 10 {
            println!("len {}: number: {}", len, nb);
        } else if len >= 100 - digit && len <= 100 {
            println!("len {}: number: {}", len, nb);
        } else if len >= 1_000 - digit && len <= 1_000 {
            println!("len {}: number: {}", len, nb);
        } else if len >= 10_000 - digit && len <= 10_000 {
            println!("len {}: number: {}", len, nb);
        } else if len >= 100_000 - digit && len <= 100_000 {
            println!("len {}: number: {}", len, nb);
        } else if len >= 1_000_000 - digit && len <= 1_000_000 {
            println!("len {}: number: {}", len, nb);
        }
        len += digit;
    }
}

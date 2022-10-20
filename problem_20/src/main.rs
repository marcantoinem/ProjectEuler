use rug::Integer;

fn digits(num: Integer) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

fn factorial(n: usize) -> Integer {
    (1..n).product()
}

fn main() {
    let num = factorial(100);
    let sum: Integer = digits(num).sum::<Integer>();
    println!("{:?}", sum);
}

use rug::{Integer, ops::Pow};

fn digits(num: Integer) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

fn main() {
    let num = Integer::from(2_usize).pow(1000);
    let sum: Integer = digits(num).sum::<Integer>();
    println!("{:?}", sum);
}

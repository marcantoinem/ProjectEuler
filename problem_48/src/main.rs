use rug::{Integer, ops::Pow};

fn self_power(n: usize) -> Integer {
    let mut sum = Integer::from(0);
    for i in 1..n {
        sum += Integer::from(i).pow(i as u32);
    }
    Integer::from(sum)
}

fn main() {
    println!("{}", self_power(1000));
}
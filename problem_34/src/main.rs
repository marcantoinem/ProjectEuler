fn digits(num: usize) -> impl Iterator<Item = usize> {
    num.to_string()
        .chars()
        .map(|d| (d.to_digit(10).unwrap()) as usize)
        .collect::<Vec<_>>()
        .into_iter()
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn main() {
    for num in 100..10_000_000 {
        let sum: usize = digits(num).map(|x| factorial(x)).sum();
        if sum == num {
            println!("{}", sum);
        }
    }
}

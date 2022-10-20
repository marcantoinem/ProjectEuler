fn digits(num: usize) -> impl Iterator<Item = usize> {
    num.to_string()
        .chars()
        .map(|d| (d.to_digit(10).unwrap()) as usize)
        .collect::<Vec<_>>()
        .into_iter()
}

fn main() {
    let mut sum = 0;
    for n in 10..1_000_000 {
        if digits(n).map(|x| x.pow(5)).sum::<usize>() == n {
            sum += n;
        }
    }
    println!("{}", sum);
}
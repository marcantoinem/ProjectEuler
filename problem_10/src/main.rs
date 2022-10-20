fn main() {
    let sum: usize = primal::Primes::all().take_while(|p| *p < 2_000_000).sum();
    println!("{}", sum);
}

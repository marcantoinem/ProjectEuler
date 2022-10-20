fn totient(factors: &Vec<(usize,u32)>) -> usize {
    factors.iter().map(|(p, k)| (p - 1) * p.pow(k - 1)).product()
}

fn defactorize(factors: &Vec<(usize,u32)>) -> usize {
    factors.iter().map(|(prime, a)| prime.pow(*a)).product()
}

fn resilience(factors: &Vec<(usize,u32)>) -> f64 {
    totient(factors) as f64 / (defactorize(factors) - 1) as f64
}

fn main() {
    let ratio = 15499_f64 / 94744_f64;
    let factors = vec![(2,3),(3,1),(5,1),(7,1),(11,1),(13,1),(17,1),(19,1),(23,1)];
    let resilience = resilience(&factors);
    println!("{} with {}", resilience - ratio, defactorize(&factors));
}

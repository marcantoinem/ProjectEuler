// Faster to calculate the common multiple by hand than writing an algorithm.
fn main() {
    let vec = vec![19,17,16,13,11,9,7,5];
    let product: usize = vec.into_iter().product();
    println!("{}", product);
}

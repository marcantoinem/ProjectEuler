// Calculated with sum properties and quadratic equation
fn diagonal_sum(n: usize) -> usize {
    2 * n * (n * (8 * n + 15) + 13) / 3 + 1
}

fn main() {
    let n = (1001 - 1) / 2;
    println!("Diagonal sum: {}", diagonal_sum(n));
}
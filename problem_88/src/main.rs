use std::time::Instant;

fn product_sum(product: usize, sum: usize, len: usize, lower_bound: usize, solution: &mut [usize; 24_001]) {
    let k = (product as isize - sum as isize + len as isize) as usize; // Size of the set
    if k <= 24_000 {
        if product < solution[k] {
            solution[k] = product;
        }
        for i in lower_bound..=(24_000 / product * 2) {
            product_sum(i*product, sum+i, len+1, i, solution); 
        } 
    }
}

fn main() {
    // The minimal solution is borned between k and 2k.
    let time = Instant::now();
    let mut solution = [24_000; 24_001];
    product_sum(1, 1, 1, 2, &mut solution);
    let mut solution: Vec<&usize> = solution[2..=12000].into_iter().collect();
    solution.sort();
    solution.dedup();
    println!("The solution is {} computed in {}ms.", solution.into_iter().sum::<usize>(), time.elapsed().as_millis());
}

use math_arsenal::digit::Digits;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn sum_digit_factorial(n: usize) -> usize {
    let digits = Digits::from(n);
    digits.into_iter().map(factorial).sum::<usize>()
}

// fn chain(n: usize) -> Vec<usize> {
//     let mut vec = vec![n];
//     for _ in 0..59 {
//         let len = vec.len() - 1;
//         let next = sum_digit_factorial(vec[len]);
//         if vec.contains(&next) {
//             return vec
//         }
//         vec.push(next);
//     }
//     vec
// }

fn chain_is_long(n: usize) -> usize {
    let mut vec = vec![n];
    for _ in 0..59 {
        let len = vec.len() - 1;
        let next = sum_digit_factorial(vec[len]);
        if vec.contains(&next) {
            return 0
        }
        vec.push(next);
    }
    1
}

fn main() {
    let sum: usize = (1..1_000_000).into_par_iter().map(chain_is_long).sum();
    println!("{}", sum);
}

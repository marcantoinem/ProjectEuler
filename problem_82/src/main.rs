use ndarray::Array2;
use std::{fs::read_to_string, cmp, time::Instant};

const LEN: usize = 80;

fn main() {
    let mut matrix = Array2::<usize>::zeros((LEN,LEN));
    let file = read_to_string("./src/p082_matrix.txt").expect("File not found");
    for (i, line) in file.lines().enumerate() {
        for (j, number) in line.split(',').enumerate() {
            let number: usize = number.parse().unwrap();
            matrix[[i,j]] = number;
        }
    }
    let instant = Instant::now();
    println!("{}", matrix);
    for j in (0..LEN-1).rev() {
        let mut sol = [0; LEN];
        sol[0] = matrix[[0,j]] + matrix[[0,j+1]];
        for i in 1..LEN {
            sol[i] = matrix[[i,j]] + cmp::min(matrix[[i,j+1]], sol[i-1]);
        }
        for i in (0..LEN-1).rev() {
            sol[i] = cmp::min(sol[i], sol[i+1] + matrix[[i,j]]);
        }
        for (i, sol) in sol.iter().enumerate() {
            matrix[[i,j]] = *sol;
        }
    }
    println!("{}", matrix);
    println!("{} found in {} µs.", matrix.column(0).iter().min().unwrap(), instant.elapsed().as_micros());
}

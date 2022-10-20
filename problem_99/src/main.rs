use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./src/p099_base_exp.txt").unwrap();
    let mut best_log = 0.0;
    for (i, line) in file.lines().enumerate() {
        let mut iter = line.split(',');
        let a = iter.next().unwrap();
        let a: usize = a.parse().unwrap();
        let b = iter.next().unwrap();
        let b: usize = b.parse().unwrap();
        let log = b as f64 * (a as f64).log10();
        if log > best_log {
            best_log = log;
            println!("{} exponent {} at line {} is the biggest to date.", a, b, i+1);
        }
    }
}

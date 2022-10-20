use std::{time::Instant, thread};
fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        if a==0 {return b};
        b %= a;
    }
    a
}

fn branch_coprime(m: usize, n: usize, solutions: &mut [usize; 1500001], nb_solutions: &mut usize) {
    if m < 867 {
        let a = m*m - n*n;
        let b = 2*m*n;
        let c = m*m + n*n;
        let sum = a + b + c;
        let mut index = sum;
        while index <= 1_500_000 {
            solutions[index] += 1;
            if solutions[index] == 1 {*nb_solutions += 1;}
            else if solutions[index] == 2 {*nb_solutions -= 1;}
            index += sum;
        }
        if 2*m >= n {
            branch_coprime(2 * m - n, m, solutions, nb_solutions);
        }
        branch_coprime(2 * m + n, m, solutions, nb_solutions);
        branch_coprime(m + 2 * n, n, solutions, nb_solutions);
    }
}

fn run() {
    let time = Instant::now();
    let mut solutions = [0;1_500_001];
    let mut nb_solutions = 0;
    branch_coprime(2, 1, &mut solutions, &mut nb_solutions);
    // for m in 2..867_usize {
    //     for n in 1..m {
    //         if (n + m) % 2 == 1 && gcd(m,n) == 1 {
    //             let a = m*m - n*n;
    //             let b = 2*m*n;
    //             let c = m*m + n*n;
    //             let sum = a + b + c;
    //             let mut index = sum;
    //             while index <= 1_500_000 {
    //                 solutions[index] += 1;
    //                 if solutions[index] == 1 {nb_solutions += 1;}
    //                 else if solutions[index] == 2 {nb_solutions -= 1;}
    //                 index += sum;
    //             }
    //         }
    //     }
    // }
    println!("{} computed in {}", nb_solutions, time.elapsed().as_secs_f64());
}

const STACK_SIZE: usize = 128 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

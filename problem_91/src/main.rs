use std::{time::Instant, thread};

// fn _is_right_triangle(x1: isize, x2: isize, y1: isize, y2: isize) -> bool {
//     let a_pow2 = x1.pow(2) + y1.pow(2);
//     let b_pow2 = x2.pow(2) + y2.pow(2);
//     let c_pow2 = (x1-x2).pow(2) + (y2-y1).pow(2);
//     if a_pow2 == 0 || b_pow2 == 0 || c_pow2 == 0 {
//         return false
//     }
//     a_pow2 == b_pow2 + c_pow2 || b_pow2 == a_pow2 + c_pow2 || c_pow2 == a_pow2 + b_pow2
// }

const SIZE: usize = 50;

// https://projecteuler.net/thread=91;page=2#6637
fn run() {
    let instant = Instant::now();
    let mut nb_sol = 0;
    let mut checked = [[false; SIZE + 1]; SIZE + 1];
    for y in 1..=SIZE/2 {
        for x in 1..=SIZE/2 {
            if !checked[y][x] {
                let mut kx = x;
                let mut ky = y;
                while kx <= SIZE && ky <= SIZE {
                    if kx.pow(2) < ky*(SIZE-ky) {
                        nb_sol += kx / y;
                    } else {
                        nb_sol += (SIZE - ky) / x;
                    }
                    checked[ky][kx] = true;
                    kx += x;
                    ky += y;
                }
            }
        }
        for kx in (SIZE / 2 + 1)..=(SIZE-y) {
            if !checked[y][kx] {
                nb_sol += 1;
            }
        }
    }
    nb_sol = nb_sol * 2 + 3*SIZE*SIZE;
    // Because we count twice each coordinate;
    println!("{} solutions in {} ms", nb_sol, instant.elapsed().as_millis());
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

// fn main() {
//     let mut nb_sol = 0;
//     for x1 in 0..=SIZE {
//         for x2 in 0..=SIZE {
//             for y1 in 0..=SIZE {
//                 for y2 in 0..=SIZE {
//                     if _is_right_triangle(x1, x2, y1, y2) {
//                         nb_sol += 1;
//                     }
//                 }
//             }
//         }
//     }
//     nb_sol /= 2;
//     println!("{}", nb_sol);
// }

use std::time::Instant;

use itertools::Itertools;

fn replace(vec: Vec<usize>, to_replace: usize, replacement: usize) -> Vec<usize> {
    let mut vec = vec;
    if let Some(index) = vec.iter().position(|&r| r == to_replace) {
        vec[index] = replacement;
    }
    vec
}

fn main() {
    let instant = Instant::now();
    let mut nb_sol = 0;
    for c1 in (0..10).combinations(6) {
        let c1 = replace(c1, 9, 6);
        for c2 in (0..10).combinations(6) {
            let c2 = replace(c2, 9, 6);
            let combinations = [
                (0, 1),
                (0, 4),
                (0, 6),
                (1, 6),
                (2, 5),
                (3, 6),
                (4, 6),
                (6, 4),
                (8, 1),
            ];
            let mut respected = true;
            for (a, b) in combinations {
                if !(c1.contains(&a) && c2.contains(&b)) && !(c2.contains(&a) && c1.contains(&b)) {
                    respected = false;
                    break;
                }
            }
            if respected {
                nb_sol += 1;
            }
        }
    }
    println!("{} computed in {}s.", nb_sol/2, instant.elapsed().as_secs_f64());
}

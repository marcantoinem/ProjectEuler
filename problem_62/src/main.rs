use std::collections::HashMap;
use math_arsenal::digit::Digits;

fn main() {
    let mut hash_map = HashMap::new();
    let mut max = (0, 0);
    let mut x: usize = 1;
    let n_permutation = 7;
    while max.0 < n_permutation {
        x += 1;
        let cube = x.pow(3);
        let mut cube = Digits::from(cube);
        cube.sort();
        let key = cube.to_nb();
        hash_map.entry(key).or_insert((0, x)).0 += 1;
        max = *hash_map.iter().max_by_key(|entry | entry.1).unwrap().1;
    }
    let first_cube  = max.1;
    while x.pow(3) < 10 * first_cube.pow(3) {
        x += 1;
        let cube = x.pow(3);
        let mut cube = Digits::from(cube);
        cube.sort();
        let key = cube.to_nb();
        hash_map.entry(key).or_insert((0, x)).0 += 1;
    }
    let mut min = usize::MAX;
    for (_, (i, x)) in hash_map {
        if i == n_permutation && x < min {
            min = x;
        }
    }
    println!("{}", min.pow(3));
}

use math_arsenal::figurate::Figurate;
use itertools::Itertools;

fn cyclic(set: Vec<Vec<usize>>) {
    let i = 0;
    for a in set[i].iter() {
        let last_two_digits = a % 100;
        for b in set[i+1].iter() {
            let first_two_digits = b / 100 % 100;
            if last_two_digits == first_two_digits {
                let last_two_digits = b % 100;
                let mut chains = vec![*a,*b];
                cyclic_r(&set, &mut chains,2, last_two_digits)
            }
        }
    }
}

fn cyclic_r(set: &Vec<Vec<usize>>, chains: &mut Vec<usize>, i: usize, last_two_digits: usize) {
    if i < 6 {
        for a in set[i].iter() {
            let first_two_digits = a / 100 % 100;
            if last_two_digits == first_two_digits {
                let last_two_digits = a % 100;
                chains.push(*a);
                cyclic_r(set, chains, i+1, last_two_digits);
            }
        }
    } else if i == 6 {
        let first_two_digits = chains[0] / 100 % 100;
        if last_two_digits == first_two_digits {
            let sum: usize = chains.iter().sum();
            println!("{:?}, sum = {}", chains, sum);
        }
    }
}

fn main() {
    let triangle: Vec<usize> = (45..141_usize).map(|x| x.triangular()).collect();
    let square: Vec<usize> = (32..100_usize).map(|x| x.square()).collect();
    let pentagon: Vec<usize> = (26..82_usize).map(|x| x.pentagonal()).collect();
    let hexagon: Vec<usize> = (23..71_usize).map(|x| x.hexagonal()).collect();
    let heptagon: Vec<usize> = (21..64_usize).map(|x| x.heptagonal()).collect();
    let octagon: Vec<usize> = (19..59_usize).map(|x| x.octagonal()).collect();
    let set = [triangle, square, pentagon, hexagon, heptagon, octagon];
    for permutation in set.into_iter().permutations(6) {
        cyclic(permutation);
    }
}

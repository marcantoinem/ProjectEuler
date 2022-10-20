use itertools::Itertools;

fn is_valid(p: [usize; 10]) -> bool {
    if p[5] == 10 || p[6] == 10 || p[7] == 10 || p[8] == 10 || p[9] == 10 {
        return false;
    }
    if p[0] > p[1] || p[0] > p[2] || p[0] > p[3] || p[0] > p[4] {
        return false;
    }
    p[0] + p[5] == p[1] + p[7]
        && p[1] + p[6] == p[2] + p[8]
        && p[2] + p[7] == p[3] + p[9]
        && p[3] + p[8] == p[4] + p[5]
}

fn digit_string(p: [usize; 10]) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        p[0], p[5], p[6], p[1], p[6], p[7], p[2], p[7], p[8], p[3], p[8], p[9], p[4], p[9], p[5]
    )
}

fn main() {
    // 5-outer then 5-inner
    let pentagon_ring: [usize; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut max: usize = digit_string([0; 10]).parse().unwrap();
    for permutation in pentagon_ring.into_iter().permutations(10) {
        let p: [usize; 10] = permutation.try_into().unwrap();
        if is_valid(p) {
            let digit_string: usize = digit_string(p).parse().unwrap();
            if digit_string > max {
                max = digit_string;
            }
        }
    }
    println!("{}", max);
}

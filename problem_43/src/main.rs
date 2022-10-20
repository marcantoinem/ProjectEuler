use itertools::Itertools;

fn concatenate(vec: &Vec<usize>) -> usize {
    let mut n = 0;
    let len = vec.len();
    for i in 0..len {
        n += vec[i] * 10_usize.pow((len - i - 1) as u32);
    }
    n
}

fn concatenate_slice(vec: &[usize]) -> usize {
    let mut n = 0;
    let len = vec.len();
    for i in 0..len {
        n += vec[i] * 10_usize.pow((len - i - 1) as u32);
    }
    n
}

fn condition(vec: &Vec<usize>) -> bool {
    if vec[0] == 0 || vec[3] % 2 == 1 || (vec[2] + vec[3] + vec[4]) % 3 != 0 || !(vec[5] == 5 || vec[5] == 0) {
        return false
    }
    if concatenate_slice(&vec[4..7]) % 7 != 0 || concatenate_slice(&vec[5..8]) % 11 != 0 || concatenate_slice(&vec[6..9]) % 13 != 0 || concatenate_slice(&vec[7..10]) % 17 != 0 {
        return false
    }
    true
}

fn main() {
    let arr = [0,1,2,3,4,5,6,7,8,9];
    let mut sum = 0;
    for n in arr.into_iter().permutations(10) {
        if condition(&n) {
            println!("{}", concatenate(&n));
            sum += concatenate(&n);
        }
    }
    println!("{}", sum);
}
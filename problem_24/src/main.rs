use core::fmt;

fn factorial(n: usize) -> usize {
    (1..=n).product::<usize>()
}

fn nth_permutation(n: usize,  set: &Vec<usize>) -> NumList {
    let mut vec = vec![];
    let len = set.len();
    let mut set = set.to_owned();
    let mut m = n - 1;
    for i in (1..len).rev() {
        let index = m / factorial(i);
        m -= index * factorial(i);
        vec.push(set.remove(index));
    }
    vec.push(set.remove(0));
    NumList(vec)
}

struct NumList(Vec<usize>);

impl fmt::Display for NumList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, element| {
            result.and_then(|_| write!(f, "{}", element))
        })
    }
}

fn main() {
    let set = vec![0,1,2,3,4,5,6,7,8,9];
    println!("{}", nth_permutation(1_000_000, &set));
}
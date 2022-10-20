// (2,2) to remove
// How many way there are to combine 2 and 2.
// C with n=4 and k=2
// Where n=4 is the length of the path and 2 the number of choice
use rug::Integer;

fn combination(n: usize, k: usize) -> Integer {
    ((n-k+1)..=n).product::<Integer>() / (1..=k).product::<Integer>()
}

fn main() {
    println!("{}", combination(40, 20));
}

use rug::{Integer, ops::Pow};

fn main() {
    let mut vec = vec![];
    for a in 2..=100 {
        for b in 2..=100 {
            let num = Integer::from(a as usize).pow(b);
            if !vec.contains(&num) {
                vec.push(num);
            }
        }
    }
    println!("{}", vec.len());
}

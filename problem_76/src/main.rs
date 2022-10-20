// Same as 31;


const NB: usize = 5;

fn main() {
    let mut ways = [0; NB + 1];
    ways[0] = 1;
    for i in 1..NB {
        for j in i..=NB {
            ways[j] += ways[j - i];
        }
    }
    println!("{}", ways[NB]);
}
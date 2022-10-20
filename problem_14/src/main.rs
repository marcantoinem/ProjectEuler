fn collatz_length(n: usize) -> usize {
    let mut length = 1;
    let mut i = n;
    loop {
        if i == 1 {
            return length
        } else if i&1 == 0 {
            i = i / 2;
        } else if i&1 == 1 {
            i = 3*i + 1;
        }
        length += 1;
    }
}

fn main() {
    let mut longest_length = 1;
    let mut best_num = 1;
    for n in 1..1_000_000 {
        let length = collatz_length(n);
        if length > longest_length {
            longest_length = length;
            best_num = n;
        }
    }
    println!("{}", best_num)
}

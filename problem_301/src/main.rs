fn main() {
    let mut count = 0;
    for n in 1_usize..=(2_usize.pow(30)) {
        if ((n) ^ (2*n) ^ (3*n)) == 0 {
            count += 1;
        }
    }
    println!("{} {}", count, 2_usize.pow(30));
}

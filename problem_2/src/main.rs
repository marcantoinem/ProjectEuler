fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut sum: usize = 2;
    while a <= 4_000_000 && b <= 4_000_000 {
        a = a + b;
        if a%2 == 0 {
            sum += a;
        }
        println!("{}", a);
        b = a + b;
        if b%2 == 0 {
            sum += b;
        }
        println!("{}", b);
    }
    println!("The sum of the even below 4 000 000 is {}", sum);
}

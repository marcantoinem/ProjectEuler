// Determine if an iterable equals itself reversed
fn is_palindrome(n: usize) -> bool {
    let mut num = n;
    let mut vec = vec![];
    while num != 0 {
        vec.push(num % 10);
        num /= 10;
    }
    let iter = vec.into_iter();
    iter.clone().eq(iter.rev())
}

fn main() {
    let mut biggest_product = 0;
    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let product = a*b;
            if is_palindrome(product) && product > biggest_product {
                biggest_product = product;
            }
        }
    }
    println!("{}", biggest_product);
}

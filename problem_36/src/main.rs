use bitvec::prelude::*;

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

fn trim<T: BitStore, O: BitOrder>(bits: &BitSlice<T, O>, to_trim: bool) -> &BitSlice<T, O> {
    let stop = |b: bool| b != to_trim;
    let front = bits.iter().by_vals().position(stop).unwrap_or(0);
    let back = bits.iter().by_vals().rposition(stop).map_or(0, |p| p + 1);
    &bits[front..back]
}

fn reverse_bits(n: usize) -> usize {
    let bits = n.view_bits::<Msb0>();
    let slice = trim(&bits, false);
    let rev = &mut slice.to_owned();
    rev.reverse();
    let rev = rev.load::<usize>();
    rev
}

fn main() {
    let mut vec = vec![];
    for k in 1..2_usize.pow(20) {
        // println!("{:32b}", k);
        let middle = (k as f64).log2().floor() as u32 + 1;
        // println!("{:32b}, middle {}", k*2_u32.pow(middle + 1) + 2_u32.pow(middle) + reverse_bits(k), middle);
        vec.push(k * 2_usize.pow(middle) + reverse_bits(k));
        vec.push(
            k * 2_usize.pow(middle + 1) + reverse_bits(k),
        );
        vec.push(
            k * 2_usize.pow(middle + 1) + 2_usize.pow(middle)+ reverse_bits(k)
        );
    }
    vec.sort();
    vec.dedup();
    let mut sum  = 0;
    for n in vec.into_iter().take_while(|x| x < &1_000_000_000_000) {
        if is_palindrome(n) {
            sum += n;
            println!("{:32b}, {}", n, n);
        }
    }
    println!("{}", sum);
}

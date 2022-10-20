use english_numbers::{convert, Formatting};

fn main() {
    let mut sum = 0;
    let fmt = Formatting {
        title_case: false,
        spaces: false,
        conjunctions: true,
        commas: false,
        dashes: false
    };
    for n in 1..=1000 {
        let str = convert(n, fmt);
        let count = str.chars().count();
        println!("{}, {}", str, count);
        sum += count;
    }
    println!("{}", sum)
}

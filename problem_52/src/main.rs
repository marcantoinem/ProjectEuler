use math_arsenal::digit::Digits;

fn multiple_are_permuted(mut digits: Digits, upper_bound: usize) -> bool {
    let nb = digits.to_nb();
    digits.sort();
    for k in 2..=upper_bound {
        let multiple = k*nb;
        let mut multiple = Digits::from(multiple);
        multiple.sort();
        if multiple != digits {
            return false
        }
    }
    true
}

fn main() {
    let first_digit = 1;
    for second_digit in 0..7 {
        // The digits must be divisible by 3 (1+second_digit+the_rest == 0 mod 3)
        for nb in ((8-second_digit)..100_000_usize).step_by(3) {
            let mut digits = Digits::from(nb);
            digits.push(second_digit);
            digits.push(first_digit);
            let nb = digits.to_nb();
            if multiple_are_permuted(digits, 6) {
                println!("{} x 2 = {}", nb, 2 * nb);
                println!("{} x 3 = {}", nb, 3 * nb);
                println!("{} x 4 = {}", nb, 4 * nb);
                println!("{} x 5 = {}", nb, 5 * nb);
                println!("{} x 6 = {}", nb, 6 * nb);
            }
        }
    }
}

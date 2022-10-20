use std::fs::read_to_string;
use std::time::Instant;

// fn to_digital(roman: &str) -> usize {
//     let mut sum = 0;
//     for char in roman.chars() {
//         sum += match char {
//             'I' => 1,
//             'V' => 5,
//             'X' => 10,
//             'L' => 50,
//             'C' => 100,
//             'D' => 500,
//             'M' => 1000,
//             _ => 0,
//         }
//     };
//     sum
// }
//
// fn minimal_roman_form(nb: usize) -> String {
//     let mut str = String::new();
//     let mut nb = nb;
//     while nb > 0 {
//         nb -= match nb {
//             1000.. => {str.push('M'); 1000},
//             500.. => {str.push('D'); 500},
//             100.. => {str.push('C'); 100},
//             50.. => {str.push('L'); 50},
//             10.. => {str.push('X'); 10},
//             5.. => {str.push('V'); 5},
//             1.. => {str.push('I'); 1},
//             _ => 0,
//         }
//     }
//     str
// }

fn main() {
    let instant = Instant::now();
    let string = read_to_string("./src/p089_roman.txt").unwrap();
    let new_string = string.replace("VIIII", "IX").replace("IIII", "IV").replace("LXXXX", "XC").replace("XXXX", "XL").replace("DCCCC", "CM").replace("CCCC", "CD");
    println!("{} characters are saved by reducing the number to their minimal form which has taken {}.", string.len() - new_string.len(), instant.elapsed().as_secs_f64());
}

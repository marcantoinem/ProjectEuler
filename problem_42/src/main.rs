use std::fs;

fn letter_to_value(letter: char) -> usize {
    match letter {
        'a' => return 1,
        'b' => return 2,
        'c' => return 3,
        'd' => return 4,
        'e' => return 5,
        'f' => return 6,
        'g' => return 7,
        'h' => return 8,
        'i' => return 9,
        'j' => return 10,
        'k' => return 11,
        'l' => return 12,
        'm' => return 13,
        'n' => return 14,
        'o' => return 15,
        'p' => return 16,
        'q' => return 17,
        'r' => return 18,
        's' => return 19,
        't' => return 20,
        'u' => return 21,
        'v' => return 22,
        'w' => return 23,
        'x' => return 24,
        'y' => return 25,
        'z' => return 26,
        _ => return 0,
    }
}

fn main() {
    let mut string = fs::read_to_string("./src/p042_words.txt").expect("Unable to read file");
    string.retain(|c| !r#"()".;:'"#.contains(c));
    let mut count = 0;
    for str in string.split(",") {
        let t = str.chars().map(|x| letter_to_value(x.to_ascii_lowercase())).sum::<usize>();
        let discriminant = 1+8*t;
        let root = (discriminant as f64).sqrt().round() as usize;
        if root * root == discriminant {
            count += 1;
        }
    }
    println!("{}", count)
}

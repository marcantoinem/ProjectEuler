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
    let mut string = fs::read_to_string("./src/p022_names.txt").expect("Unable to read file");
    string.retain(|c| !r#"()".;:'"#.contains(c));
    let mut vector: Vec<&str> = string.split(",").collect();
    vector.sort();
    let mut sum = 0;
    for (i, name) in vector.iter().enumerate() {
        let score: usize = (i + 1) * name.chars().map(|x| letter_to_value(x.to_ascii_lowercase())).sum::<usize>();
        sum += score;
    }
    println!("{}", sum);
}
use std::{fs, collections::HashMap};

use itertools::Itertools;

fn find_key(file: &String) -> [u8;3] {
    let mut frequency: [HashMap<u8, usize>;3] = [HashMap::new(), HashMap::new(), HashMap::new()];
    for chunk in &file.split(',').chunks(3) {
        for (i, nb) in chunk.enumerate() {
            let nb: u8 = nb.parse().unwrap();
            *frequency[i].entry(nb).or_insert(0) += 1;
        }
    }
    let a = frequency[0].iter().max_by_key(|entry | entry.1).unwrap().0 ^ 32;
    let b = frequency[1].iter().max_by_key(|entry | entry.1).unwrap().0 ^ 32;
    let c = frequency[2].iter().max_by_key(|entry | entry.1).unwrap().0 ^ 32;
    [a,b,c]
}

fn decrypt(key: [u8;3], file: &String) -> String {
    let mut str = String::new();
    for chunk in &file.split(',').chunks(3) {
        for (i, nb) in chunk.enumerate() {
            let mut nb: u8 = nb.parse().unwrap();
            nb ^= key[i];
            str.push(nb as char);
        }
    }
    str
}

fn sum_ascii(str: String) -> usize {
    str.chars().map(|x| x as usize).sum()
}

fn main() {
    let file = fs::read_to_string("src/p059_cipher.txt").unwrap();
    let key = find_key(&file);
    let mut key_str = String::new();
    for n in key {
        key_str.push(n as char);
    }
    println!("{}", key_str);
    let message = decrypt(key, &file);
    println!("{}", message);
    let sum = sum_ascii(message);
    println!("{}", sum);
}

use digits_iterator::*;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
    time::Instant,
};

fn is_square(n: usize) -> bool {
    let sqrt = (n as f64).sqrt();
    sqrt == sqrt.ceil()
}

fn find_anagram(new_map: BTreeMap<usize, Vec<(&str, &str)>>) -> usize {
    for (len, vec) in new_map.iter().rev() {
        for (word_a, word_b) in vec {
            let bword_a = word_a.as_bytes();
            let bword_b = word_b.as_bytes();
            let born_inf = 10_f64.powf((*len - 1) as f64 / 2.0).ceil() as usize;
            let born_sup = 10_f64.powf(*len as f64 / 2.0).ceil() as usize;
            for x in (born_inf..born_sup).rev() {
                let mut hashmap = HashMap::new();
                let square = x.pow(2);
                let digits: Vec<_> = square.digits().collect();
                if digits.iter().sorted().dedup().count() == digits.len() {
                    for (index, digit) in digits.into_iter().enumerate() {
                        hashmap.insert(bword_a[index], digit as usize);
                    }
                    if hashmap.get(&bword_b[0]).unwrap() != &0 {
                        let mut other_nb = 0;
                        for (index, char) in bword_b.iter().enumerate() {
                            let number = hashmap.get(char).unwrap();
                            other_nb += number * 10_usize.pow((len - index - 1) as u32)
                        }
                        println!("{}: {}, {}: {}", word_a, square, word_b, other_nb);
                        if is_square(other_nb) {
                            return std::cmp::max(other_nb, square);
                        }
                    }
                }
            }
        }
    };
    0
}

fn main() {
    let instant = Instant::now();
    let file = read_to_string("./p098_words.txt").unwrap();
    let mut map = BTreeMap::new();
    let filtered = file.replace('"', "");
    for word in filtered.split(',') {
        let len = word.len();
        let word_sorted = word.chars().sorted().collect::<Vec<char>>();
        let word_sorted2 = word_sorted.to_owned();
        let word = word.clone();
        map.entry(len).or_insert(vec![]).push((word_sorted2, word));
    }
    let mut new_map = BTreeMap::new();
    for (word_len, mut vec) in map.into_iter() {
        vec.sort_unstable();
        let mut anagrams = vec![];
        let len = vec.len();
        let mut index = 0;
        while index < len - 1 {
            if vec[index].0 == vec[index + 1].0 {
                anagrams.push((vec[index].1, vec[index + 1].1));
                index += 2;
            } else {
                index += 1;
            }
        }
        new_map.insert(word_len, anagrams);
    }
    let best: usize = find_anagram(new_map); 
    println!(
        "The answer is {} computed in {}s",
        best,
        instant.elapsed().as_secs_f64()
    );
}

use std::fs;

fn main() {
    let file = fs::read_to_string("./src/p079_keylog.txt").expect("Failed to open the file");
    let mut codes: Vec<Vec<u32>> = file.lines().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect()).collect(); 
    codes.sort();
    codes.dedup();
    let mut different_nb: Vec<u32> = vec![];
    for code in codes.iter() {
        for nb in code {
            if !different_nb.contains(nb) {
                different_nb.push(*nb);
            }
        }
    }
     
    let mut topologic = vec![];
    for nb in different_nb.iter() {
        let mut after = vec![];
        for code in codes.iter() {
            if &code[0] == nb {
                if !after.contains(&code[1]) {after.push(code[1]);}
                if !after.contains(&code[2]) {after.push(code[2]);}
            } else if &code[1] == nb && !after.contains(&code[2]) {after.push(code[2]);}
        }
        topologic.push((nb, after));
    }    
    topologic.sort_by_key(|x| x.1.len());
    let passcode: u32 = topologic.into_iter().enumerate().map(|(i, (x, _))| *x * 10_u32.pow(i as u32)).sum();
    println!("{:?}", passcode);
}

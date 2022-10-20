use evalexpr::eval;
use itertools::Itertools;

fn main() {
    let mut best = 0;
    for a in 0..7 {
        for b in a+1..8 {
            for c in b+1..9 {
                for d in c+1..10 {
                    let mut sol = vec![];
                    for permutation in [a, b, c, d].into_iter().permutations(4) {
                        let a = permutation[0];
                        let b = permutation[1];
                        let c = permutation[2];
                        let d = permutation[3];
                        for op_1 in ['+', '-', '*', '/'] {
                            for op_2 in ['+', '-', '*', '/'] {
                                for op_3 in ['+', '-', '*', '/'] {
                                    let str = format!("{}.0 {} {}.0 {} {}.0 {} {}.0", a, op_1, b, op_2, c, op_3, d); 
                                    if let Ok(value) = eval(&str) {
                                        let value = value.as_float().unwrap();
                                        if value.ceil() == value {
                                            let value = value as usize;
                                            if !sol.contains(&value) {
                                                sol.push(value);
                                            }
                                        }
                                    }
                                    let str = format!("{}.0 {} ({}.0 {} {}.0) {} {}.0", a, op_1, b, op_2, c, op_3, d);
                                    if let Ok(value) = eval(&str) {
                                        let value = value.as_float().unwrap();
                                        if value.ceil() == value {
                                            let value = value as usize;
                                            if !sol.contains(&value) {
                                                sol.push(value);
                                            }
                                        }
                                    }
                                    let str = format!("{}.0 {} {}.0 {} ({}.0 {} {}.0)", a, op_1, b, op_2, c, op_3, d); 
                                    if let Ok(value) = eval(&str) {
                                        let value = value.as_float().unwrap();
                                        if value.ceil() == value {
                                            let value = value as usize;
                                            if !sol.contains(&value) {
                                                sol.push(value);
                                            }
                                        }
                                    }
                                    let str = format!("{}.0 {} ({}.0 {} {}.0 {} {}.0)", a, op_1, b, op_2, c, op_3, d); 
                                    if let Ok(value) = eval(&str) {
                                        let value = value.as_float().unwrap();
                                        if value.ceil() == value {
                                            let value = value as usize;
                                            if !sol.contains(&value) {
                                                sol.push(value);
                                            }
                                        }
                                    }
                                    let str = format!("({}.0 {} {}.0) {} ({}.0 {} {}.0)", a, op_1, b, op_2, c, op_3, d); 
                                    if let Ok(value) = eval(&str) {
                                        let value = value.as_float().unwrap();
                                        if value.ceil() == value {
                                            let value = value as usize;
                                            if !sol.contains(&value) {
                                                sol.push(value);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }                    
                    sol.sort();
                        let mut len = 1;
                        while sol[len] == len {
                            len += 1;
                        }
                        if len > best {
                            best = len;
                            println!("{} with {}{}{}{} len: {} sol: {:?}", len, a, b, c, d, sol.len(), sol);
                        }
                }
            }
        }
    }
}

// use std::collections::HashMap;

fn move_(n: usize, moves: &mut Vec<(usize, usize)>, from: usize, to: usize, via: usize) {
    if n > 0 {
        move_(n - 1, moves, from, via, to);
        moves.push((from, to));
        move_(n - 1, moves, via, to, from);
    }
}

fn main() {
    let n = 3;
    let mut moves = vec![];
    move_(n, &mut moves, 0, 2, 1);
    let mut stacks = [n+1, 0, 0];
    let mut sum = 0;
    // let mut frequency: HashMap<[usize; 3], usize> = HashMap::new();
    for (i, (from, to)) in moves.into_iter().enumerate() {
        
        if stacks[0] ^ stacks[1] ^ stacks[2] == 0 {
            // let mut copy = stacks.clone();
            // copy.sort();
            // *frequency.entry(copy).or_insert(0) += 1;
            // println!("{:?} is xor 0", copy);
            sum += i;
        } else {
            // let mut copy = stacks.clone();
            // copy.sort();
            // println!("{:?}", copy);
        }
        
        stacks[from] -= 1;
        stacks[to] += 1;
    }
    let mut copy = stacks.clone();
    copy.sort();
    // println!("{:?}", copy);
    // println!("n: {}, frequency: {:?}", n, frequency);
    println!("{}", sum);
}

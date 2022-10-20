use rand::Rng;

const SAMPLES: usize = 42_000_000;


fn main() {
    let mut rng = rand::thread_rng();
    let mut board = [0_usize; 40];
    let mut consecutive_double = 0;
    let mut pos = 0;
    let mut cc = 0; // Community Chest
    let mut ch = 0; // Chance
    let cc_card = [0, 10];
    let ch_card = [0, 10, 11, 24, 39, 5];
    for _ in 0..SAMPLES {
        let roll1 = rng.gen_range(1..=4);
        let roll2 = rng.gen_range(1..=4);
        if roll1 == roll2 && consecutive_double == 2 {
            pos = 10;
            board[10] += 1;
            consecutive_double = 0;
            continue
        } else if roll1 == roll2 {
            consecutive_double += 1;
        } else {
            consecutive_double = 0;
        }
        pos = (pos + roll1 + roll2) % 40;
        
        if pos == 7 || pos == 22 || pos == 36 {
            ch = (ch + 1) % 16;
            match ch {
                0 | 1 | 2 | 3 | 4 | 5 => pos = ch_card[ch],
                6 | 7 => {
                    if pos == 7 { pos = 15 }
                    if pos == 2 { pos = 25 }
                    if pos == 36 { pos = 5 }
                },
                8 => pos = if pos == 22 {28} else {12},
                9 => pos -= 3,
                _ => (),
            }
        }
        
        if pos == 2 || pos == 17 || pos == 33 {
            cc = (cc + 1) % 16;
            if cc < 2 {
                pos = cc_card[cc];
            }
        }

        if pos == 30 {
            pos = 10;
        }

        board[pos] += 1;
    }
    let mut iboard: [(usize, usize); 40] = board.into_iter().enumerate().collect::<Vec<(usize, usize)>>().try_into().unwrap();
    iboard.sort_by_key(|x| x.1);
    iboard.reverse();
    println!("{}{}{}", iboard[0].0, iboard[1].0, iboard[2].0);
}

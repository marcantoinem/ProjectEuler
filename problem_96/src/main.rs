use std::fs::read_to_string;
use std::time::Instant;
use sudoku::Sudoku;

fn main() {
    let instant = Instant::now();
    let file = read_to_string("./src/p096_sudoku.txt").unwrap().replace('\n', "");
    let mut index = 0;
    let mut sum = 0;
    for _sudoku_number in 0..50 {
        index += 7;
        let start_index = index;
        index += 81;
        let end_index = index;
        let str = &file[start_index..end_index];
        let sudoku = Sudoku::from_str_line(str).unwrap();
        if let Some(solution) = sudoku.solve_unique() {
            let array = solution.to_bytes();
            let key = 100 * array[0] as usize + 10 * array[1] as usize + array[2] as usize;
            println!("{:?}", key);
            sum += key;
        }
    }
    println!("The sum is {} computed in {}s.", sum, instant.elapsed().as_secs_f64());
}

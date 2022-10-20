use nalgebra::SMatrix;
use std::{fs::read_to_string, time::Instant};

const LEN: usize = 80;
type Matrix = SMatrix<usize, LEN, LEN>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Square {
    x: usize,
    y: usize,
    value: usize,
}

impl Square {
    fn from(matrix: &Matrix, x: usize, y: usize) -> Option<Square> {
        if let Some(value) = matrix.get((x, y)) {
            let value = *value;
            Some(Square { x, y, value })
        } else {
            None
        }
    }
    fn with_cost(&self, cost: usize) -> SquareCost {
        SquareCost{
            square: *self,
            cost,
        }
    }
}

#[derive(Copy, Clone)]
struct SquareCost {
    square: Square,
    cost: usize,
}

impl PartialEq for SquareCost {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}

impl SquareCost {
    // h is the estimation cost
    fn f(&self, min: &usize) -> usize {
        self.cost + min*((LEN - 1 - self.square.x) + (LEN - 1 - self.square.y))
    }
    fn push_adjacent(&self, matrix: &Matrix, open: &mut Vec<Self>, close: &Vec<Self>) {
        for (x,y) in [(1,0), (-1,0), (0,1), (0,-1)] {
            if let Some(square) = Square::from(matrix, (self.square.x as isize + x) as usize, (self.square.y as isize + y) as usize) {
                let square = square.with_cost(self.cost + square.value); 
                if !open.contains(&square) && !close.contains(&square) {
                    open.push(square);
                }
            }
        }
    } 
}

fn main() {
    let file = read_to_string("./src/p083_matrix.txt").expect("Can't access the file.");
    let len = file.len() - 1;
    let vec: Vec<usize> = file[..len]
        .replace('\n', ",")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let min = &vec.iter().min().unwrap().clone();
    // The matrix will be transposed, but it doesn't matter.
    let matrix = Matrix::from_vec(vec);
    let time = Instant::now();
    let sq1 = Square::from(&matrix, 0, 0).unwrap();
    let sq1 = sq1.with_cost(sq1.value);
    let mut open = vec![sq1];
    let mut close = vec![];
    let sum = loop {
        let mut iter = open.iter().copied().enumerate();
        let mut best_square = iter.next().unwrap();
        for square in iter {
            if square.1.f(min) < best_square.1.f(min) {
                best_square = square;
            }
        }
        if best_square.1.square == Square::from(&matrix, LEN - 1, LEN - 1).unwrap() {
            break best_square.1.cost;
        }
        open.remove(best_square.0);
        close.push(best_square.1);
        best_square.1.push_adjacent(&matrix, &mut open, &close);
    };
    println!("{} in {} microseconds", sum, time.elapsed().as_micros());
}

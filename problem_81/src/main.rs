use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct Losange(Vec<Vec<usize>>);

impl Losange {
    fn from_string(string: &str) -> Self {
        let mut matrix = vec![];
        let string = string.replace(&['\n'], ",");
        let total = string.split(",").count();
        let n = (total as f64).sqrt() as isize;
        for number in string.split(",") {
            if let Ok(number) = number.parse() {
                matrix.push(number);
            }
        }
        let mut losange = vec![];
        for row in 0..n {
            let mut vec = vec![];
            let num_element = -(row - n + 1).abs() + n;
            for x in 0..num_element {
                let index = num_element + x * (n - 1) - 1;
                vec.push(matrix[index as usize]);
            }
            losange.push(vec);
        }
        for row in n..(n * 2 - 1) {
            let mut vec = vec![];
            let num_element = -(row - n + 1).abs() + n;
            for x in 0..num_element {
                let index = num_element + x * (n - 1) + (row - n + 1) * (n + 1) - 1;
                vec.push(matrix[index as usize]);
            }
            losange.push(vec);
        }
        Losange(losange)
    }
    fn from_file(address: &str) -> Self {
        let mut file = File::open(address).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Error while reading file");
        Self::from_string(&data)
    }
    fn min_path_sum(self) -> usize {
        let mut losange = self;
        let n = (losange.0.len() + 1) / 2;
        losange.0[2*n - 3][0] += losange.0[2*n - 2][0];
        losange.0[2*n - 3][1] += losange.0[2*n - 2][0];
        for line in ((n-1)..(2*n - 3)).rev() {
            let num_element = losange.0[line].len() - 1;
            losange.0[line][0] += losange.0[line + 1][0];
            for i in 1..(num_element) {
                if losange.0[line + 1][i] < losange.0[line + 1][i - 1] {
                    losange.0[line][i] += losange.0[line + 1][i];
                } else {
                    losange.0[line][i] += losange.0[line + 1][i - 1];
                }
            }
            losange.0[line][num_element] += losange.0[line + 1][num_element - 2];
        }
        for line in (0..(n - 1)).rev() {
            let num_element = losange.0[line].len();
            for i in 0..num_element {
                if losange.0[line + 1][i] < losange.0[line + 1][i + 1] {
                    losange.0[line][i] += losange.0[line + 1][i];
                } else {
                    losange.0[line][i] += losange.0[line + 1][i + 1];
                }
            }
        }
        losange.0[0][0]
    }
}

fn main() {
    let los = Losange::from_file("./src/small_matrix.txt");
    println!("{:?}", los.min_path_sum());
}

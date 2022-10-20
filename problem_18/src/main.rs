use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Pyramid(Vec<Vec<usize>>);

impl Pyramid {
    fn from_string(string: &str) -> Pyramid {
        let mut pyramid = vec![];
        for line in string.split("\n") {
            let vec = line.split(" ").map(|ch| ch.parse().unwrap()).collect();
            pyramid.push(vec);
        }
        Pyramid(pyramid)
    }
    fn from_file(address: &str) -> Pyramid {
        let mut file = File::open(address).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Error while reading file");
        Pyramid::from_string(&data)
    }
}

fn main() {
    // let mut pyramid = Pyramid::from_string(
    //     "75\n95 64\n17 47 82\n18 35 87 10\n20 04 82 47 65\n19 01 23 75 03 34\n88 02 77 73 07 63 67\n99 65 04 28 06 16 70 92\n41 41 26 56 83 40 80 70 33\n41 48 72 33 47 32 37 16 94 29\n53 71 44 65 25 43 91 52 97 51 14\n70 11 33 28 77 73 17 78 39 68 17 57\n91 71 52 38 17 14 91 43 58 50 27 29 48\n63 66 04 68 89 53 67 30 73 16 69 87 40 31\n04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
    // );
    // let mut pyramid = Pyramid::from_string(
    //     "3\n7 4\n2 4 6\n8 5 9 3",
    // );
    let mut pyramid = Pyramid::from_file(
        "/home/lemark/Projets/rust/Projet Euler/e_2022_18/src/p067_triangle.txt",
    );
    for line in (0..(pyramid.0.len() - 1)).rev() {
        for i in 0..pyramid.0[line].len() {
            if pyramid.0[line + 1][i] > pyramid.0[line + 1][i + 1] {
                pyramid.0[line][i] += pyramid.0[line + 1][i];
            } else {
                pyramid.0[line][i] += pyramid.0[line + 1][i + 1];
            }
        }
    }

    println!("{:?}", pyramid);
}

use std::io;
use std::io::{BufRead, BufReader, Read};

fn read<R: Read>(io: R) -> Vec<(f64, f64)> {
    let reader = BufReader::new(io);
    return reader.lines()
        .skip(1)
        .map(|line| {
            let split: Vec<f64> = line.unwrap()
                .split(" ")
                .map(|val| val.parse().unwrap())
                .collect();
            return (split[0], split[1]);
        })
        .collect();
}

fn main() {
    let vec = read(io::stdin());
    println!("{:#?}", vec.iter().map(|&(v1, v2)| v1 * v2).sum::<f64>());
}


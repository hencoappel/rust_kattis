use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<(i64, i64)>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .skip(1)
        .map(|line| {
            let split: Vec<i64> = line.unwrap()
                .split(" ")
                .map(|val| val.parse().unwrap())
                .collect()
                .unwrap();
            (&split[0], &split[1]);
        })
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let vec = read(File::open(filename).unwrap());
    println!("{}", vec);
}


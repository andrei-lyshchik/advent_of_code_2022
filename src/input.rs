use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap())
}

mod book;
mod errors;
mod header;
mod lz77;
use book::parse_book;

const FILE_PATH: &str = "./data/ex.mobi";

pub fn main() {
    let buffer = std::fs::read(FILE_PATH).expect("Couldn't read file");

    let res = parse_book(&buffer).expect("Error");
    println!("{}", res);
}

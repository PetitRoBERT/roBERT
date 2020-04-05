mod book;
mod errors;
mod header;
mod lz77;
use book::parse_book;

const FILE_PATH: &str = "./data/book_example.mobi";

pub fn main() {
    let res = parse_book(FILE_PATH).expect("Error");
    println!("{}", res);
}

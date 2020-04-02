mod book;
mod header;
mod lz77;
use book::parse_book;

const FILE_PATH: &str = "./data/ex.mobi";

pub fn main() {
    parse_book(FILE_PATH);
}

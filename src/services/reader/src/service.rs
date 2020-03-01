mod protos;
mod server;
#[cfg(test)]
mod tests;

use server::reader_server::create_server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_server()
}

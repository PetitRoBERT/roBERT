use crate::protos::reader::{reader_client::ReaderClient, ReadRequest};
use crate::server::reader_server::create_server;
use async_stream;
use mobi::Mobi;
use std::fs;
use tonic::Request;

const TEST_HOSTNAME: &str = "127.0.0.1";
const TEST_PORT: usize = 55052;
const CHUNK_SIZE: usize = 10 * 1000; // 10kB
const MOBI_PATH: &str = "./src/tests/data/example.mobi";

fn get_book() -> Vec<u8> {
    fs::read(MOBI_PATH).expect("Couldn't open the book file.")
}

fn get_mobi() -> Mobi {
    Mobi::from_path(MOBI_PATH).expect("Couldn't read the mobi file")
}

#[tokio::test]
async fn reader_server_comm() {
    std::thread::spawn(move || {
        let _ = create_server(TEST_HOSTNAME, TEST_PORT);
    });

    let expected_mobi = get_mobi();

    let mut client: ReaderClient<_> =
        ReaderClient::connect(format!("http://{}:{}", TEST_HOSTNAME, TEST_PORT))
            .await
            .expect("Couldn't create the client");

    let outbound = async_stream::stream! {
        for (i, chunk) in get_book().chunks(CHUNK_SIZE).enumerate() {
            yield ReadRequest {
                chunk: chunk.to_vec()
            };
        }
    };

    let response = client
        .receive_mobi(Request::new(outbound))
        .await
        .expect("l");
    let mut inbound = response.into_inner();

    let mut content = Vec::new();
    while let Some(chunk) = inbound.message().await.expect("o") {
        content.append(&mut chunk.book.expect("No book found.").chunked_content);
    }

    assert_eq!(expected_mobi.content, content);
}

use crate::protos::reader::{reader_client::ReaderClient, ReadRequest};
use crate::server::reader_server::create_server;
use futures::stream;
use tonic::Request;

#[tokio::test]
async fn reader_server_comm() {
    std::thread::spawn(move || {
        let _ = create_server();
    });

    let mut client: ReaderClient<_> = ReaderClient::connect("htpp://localhost:50050")
        .await
        .expect("Couldn't create the client");

    let request_1 = ReadRequest {
        chunk: vec![1, 2, 4],
    };

    let request_2 = ReadRequest {
        chunk: vec![5, 6, 8],
    };

    let requests = vec![request_1, request_2];

    let streamed_request = Request::new(stream::iter(requests));

    let resp = client
        .receive_mobi(streamed_request)
        .await
        .expect("Error while sending the streamed request.")
        .into_inner();

    assert_eq!(resp.message, "Successfully processed 6 bytes sized file.")
}

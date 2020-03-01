use crate::protos::reader::{reader_client::ReaderClient, ReadRequest};
use crate::server::reader_server::create_server;

#[tokio::test]
async fn reader_server_comm() {
    std::thread::spawn(move || {
        let _ = create_server();
    });

    let mut client: ReaderClient<_> = ReaderClient::connect("htpp://localhost:50050")
        .await
        .expect("Couldn't create the client");

    let request = tonic::Request::new(ReadRequest { a: 3 });
    let resp = client
        .read_mobi(request)
        .await
        .expect("Error while sending the request")
        .into_inner();

    assert_eq!(resp.b, 1)
}

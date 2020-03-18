use crate::protos::reader::{
    reader_server::{Reader, ReaderServer},
    MobiBook, ReadRequest, ReadResponse,
};
use futures::{Stream, StreamExt};
use mobi::Mobi;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};

const DEFAULT_CAPACITY: usize = 500 * 1000; // 500kB
const DEFAULT_CHUNK_SIZE: usize = 100 * 1000; // 100kB

#[derive(Debug, Default)]
pub struct RobertReader {}

#[tonic::async_trait]
impl Reader for RobertReader {
    type ReceiveMOBIStream =
        Pin<Box<dyn Stream<Item = Result<ReadResponse, Status>> + Send + Sync + 'static>>;

    async fn receive_mobi(
        &self,
        request: Request<tonic::Streaming<ReadRequest>>,
    ) -> Result<Response<Self::ReceiveMOBIStream>, Status> {
        let mut input_stream = request.into_inner();

        let mut full_bytes = Vec::with_capacity(DEFAULT_CAPACITY);
        let mut total_counted_size = 0;

        let output = async_stream::try_stream! {
            while let Some(read_request) = input_stream.next().await {
                let read_request: ReadRequest = read_request?;
                let mut bytes: Vec<u8> = read_request.chunk;
                total_counted_size += bytes.len();
                full_bytes.append(&mut bytes);
            }

            // Now decode the received book
            let mobi_book = Mobi::new(&full_bytes)?;

            // Split the decoded book into useful pieces:
            let Mobi {
                content, // Vec[u8] holding the content of the book
                ..
            } = mobi_book;

            let chunks = content.chunks(DEFAULT_CHUNK_SIZE);

            let message = format!(
                "Successfully processed {} bytes sized file.",
                total_counted_size
            );

            for chunk in chunks {
                yield ReadResponse {
                    message: message.clone(),
                    book: Some(MobiBook {
                        chunked_content: chunk.to_vec(),
                    }),
                };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::ReceiveMOBIStream))
    }
}

#[tokio::main]
pub async fn create_server(hostname: &str, port: usize) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", hostname, port).parse()?;
    let reader = RobertReader::default();

    Server::builder()
        .add_service(ReaderServer::new(reader))
        .serve(addr)
        .await?;

    Ok(())
}

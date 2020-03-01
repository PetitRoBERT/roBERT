use crate::protos::reader::{
    reader_server::{Reader, ReaderServer},
    ReadRequest, ReadResponse,
};
use futures::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

const DEFAULT_CAPACITY: usize = 500; // 500kB

#[derive(Debug, Default)]
pub struct RobertReader {}

#[tonic::async_trait]
impl Reader for RobertReader {
    async fn receive_mobi(
        &self,
        request: Request<tonic::Streaming<ReadRequest>>,
    ) -> Result<Response<ReadResponse>, Status> {
        // let metadata = request.metadata();
        // let total_size_request = metadata
        //     .get("total_size".to_string())
        //     .map(|ascii_value| ascii_value.to_str())
        //     .map(|str_value| str_value.parse::<usize>())
        //     .or(DEFAULT_CAPACITY);
        let mut input_stream = request.into_inner();
        let mut request_summary = ReadResponse::default();

        let mut full_bytes = Vec::with_capacity(DEFAULT_CAPACITY);
        let mut total_counted_size = 0;
        while let Some(read_request) = input_stream.next().await {
            let read_request = read_request?;
            let mut bytes: Vec<u8> = read_request.chunk;
            total_counted_size += bytes.len();
            full_bytes.append(&mut bytes);
        }

        let message = format!(
            "Successfully processed {} bytes sized file.",
            total_counted_size
        );
        request_summary.message = message;

        Ok(Response::new(request_summary))
    }
}

#[tokio::main]
pub async fn create_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    let reader = RobertReader::default();

    Server::builder()
        .add_service(ReaderServer::new(reader))
        .serve(addr)
        .await?;

    Ok(())
}

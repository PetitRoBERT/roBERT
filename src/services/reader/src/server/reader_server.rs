use crate::protos::reader::{
    reader_server::{Reader, ReaderServer},
    ReadRequest, ReadResponse,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct RobertReader {}

#[tonic::async_trait]
impl Reader for RobertReader {
    async fn read_mobi(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadResponse>, Status> {
        println!("{:?}", request);
        let reply = ReadResponse { b: 1 };
        Ok(Response::new(reply))
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

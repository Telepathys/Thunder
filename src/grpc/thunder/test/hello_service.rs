pub mod thunder {
    tonic::include_proto!("thunder");
}
use thunder::{hello_server, HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct HelloService;

#[tonic::async_trait]
impl hello_server::Hello for HelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;
        let message = format!("Hello, {}!", name);
        let response = HelloResponse { message };

        Ok(Response::new(response))
    }
}
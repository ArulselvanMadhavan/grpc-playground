#[macro_use]
extern crate log;
#[path = "../utils/log_util.rs"]
mod log_util;
use grpcio::{ChannelBuilder, EnvBuilder};
use grpcio_proto::example::helloworld::HelloRequest;
use grpcio_proto::example::helloworld_grpc::GreeterClient;
use std::sync::Arc;

pub fn main() {
    let _guard = log_util::init_log(Some("client.log".to_string()));
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("127.0.0.1:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}

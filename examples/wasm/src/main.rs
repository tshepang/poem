use std::path::PathBuf;

use clap::Parser;
use poem::{listener::TcpListener, Server};
use poem_wasmhandler::WasmEndpointBuilder;

#[derive(Parser)]
struct Options {
    /// Wasm file path
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let options: Options = Options::parse();
    let wasm = std::fs::read(options.file).unwrap();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            WasmEndpointBuilder::new(wasm)
                .udf("env", "udf_add", |a: i32, b: i32| a + b)
                .build()
                .unwrap(),
        )
        .await
}

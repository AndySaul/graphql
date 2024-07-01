use clap::Parser;
use log::error;

use graphql::logger;
use graphql::server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// GraphQL port
    #[arg(short, long, default_value_t = 54321, value_name("GRAPHQL PORT"))]
    port: u16,
}

#[tokio::main]
async fn main() {
    logger::init();

    let args = Args::parse();

    if let Err(e) = server::run(args.port).await {
        error!("{e}");
    }
}

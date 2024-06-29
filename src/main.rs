use anyhow::Result;
use clap::Parser;

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
async fn main() -> Result<()> {
    logger::init();

    let args = Args::parse();

    server::run(args.port).await?;

    Ok(())
}

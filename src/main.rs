use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::*;
use clap::Parser;
use log::info;
use poem::{listener::TcpListener, web::Html, *};

use graphql::logger;

use std::net::{Ipv4Addr, SocketAddr};

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let address: SocketAddr = (Ipv4Addr::LOCALHOST, args.port).into();
    info!("GraphiQL: http://{address}");

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    let listener = TcpListener::bind(address);

    Server::new(listener).run(app).await?;
    Ok(())
}

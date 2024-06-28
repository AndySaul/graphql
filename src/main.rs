use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::*;
use env_logger::Env;
use log::info;
use poem::{listener::TcpListener, web::Html, *};

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

fn init_logger() {
    env_logger::init_from_env(
        Env::default()
            .filter_or("GRAPHQL_APP_LOG_LEVEL", "info")
            .write_style_or("GRAPHQL_APP_LOG_STYLE", "always"),
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let port: u16 = 55555;
    let address: SocketAddr = (Ipv4Addr::LOCALHOST, port).into();
    info!("GraphiQL: http://{address}");

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind(address)).run(app).await?;
    Ok(())
}

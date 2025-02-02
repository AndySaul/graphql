use anyhow::Result;
use async_graphql::http::GraphiQLSource;
use async_graphql_poem::GraphQL;
use log::info;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

use super::schema;

use std::net::{Ipv4Addr, SocketAddr};

#[handler]
fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

/// Creates and runs a GraphQL server on the given port
///
/// # Errors
///
/// Propagates errors from poem server
pub async fn run(port: u16) -> Result<()> {
    let address: SocketAddr = (Ipv4Addr::LOCALHOST, port).into();
    info!("GraphiQL: http://{address}");
    let listener = TcpListener::bind(address);

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema::new())));

    Server::new(listener).run(app).await?;
    Ok(())
}

use anyhow::Result;
use async_graphql::http::GraphiQLSource;
use async_graphql_poem::*;
use log::info;
use poem::{listener::TcpListener, web::Html, *};

use super::schema;

use std::net::{Ipv4Addr, SocketAddr};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

pub async fn run(port: u16) -> Result<()> {
    let address: SocketAddr = (Ipv4Addr::LOCALHOST, port).into();
    info!("GraphiQL: http://{address}");

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema::new())));
    let listener = TcpListener::bind(address);

    Server::new(listener).run(app).await?;
    Ok(())
}

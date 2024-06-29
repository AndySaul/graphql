use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::*;
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

pub async fn run(port: u16) -> Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let address: SocketAddr = (Ipv4Addr::LOCALHOST, port).into();
    info!("GraphiQL: http://{address}");

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    let listener = TcpListener::bind(address);

    Server::new(listener).run(app).await?;
    Ok(())
}

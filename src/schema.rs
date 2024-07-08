use anyhow::{bail, Result};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;
use tokio::sync::RwLock;

pub struct Query;

#[Object]
impl Query {
    #[allow(clippy::unused_async)]
    async fn hello(&self) -> &'static str {
        "world"
    }

    async fn value(&self, context: &Context<'_>) -> Result<i32> {
        match context.data::<RwLock<i32>>() {
            Ok(x) => Ok(*x.read().await),
            Err(e) => bail!("Cannot get GraphQL context: {e:?}"),
        }
    }
}

/// Creates a new GraphQL endpoint for poem server
#[must_use]
pub fn new() -> GraphQL<Schema<Query, EmptyMutation, EmptySubscription>> {
    let value = RwLock::new(42);
    GraphQL::new(
        Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(value)
            .finish(),
    )
}

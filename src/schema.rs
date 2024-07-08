use anyhow::{bail, Result};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;
use tokio::sync::RwLock;

pub struct Query;

#[Object]
impl Query {
    #[allow(clippy::unused_async)]
    /// Hello world example
    async fn hello(&self) -> &'static str {
        "world"
    }

    /// Retrieves the mutable value
    async fn value(&self, context: &Context<'_>) -> Result<i32> {
        match context.data::<RwLock<i32>>() {
            Ok(lock) => Ok(*lock.read().await),
            Err(e) => bail!("Cannot get GraphQL context: {e:?}"),
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// Sets a new value
    async fn set_value(&self, context: &Context<'_>, value: i32) -> Result<bool> {
        match context.data::<RwLock<i32>>() {
            Ok(lock) => {
                *lock.write().await = value;
                Ok(true)
            }
            Err(e) => bail!("Cannot get GraphQL context: {e:?}"),
        }
    }
}

/// Creates a new GraphQL endpoint for poem server
#[must_use]
pub fn new() -> GraphQL<Schema<Query, Mutation, EmptySubscription>> {
    let value = RwLock::new(42);
    GraphQL::new(
        Schema::build(Query, Mutation, EmptySubscription)
            .data(value)
            .finish(),
    )
}

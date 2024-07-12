use anyhow::Result;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;
use tokio::sync::RwLock;

type Val = RwLock<i32>;
const VALID_DATA: &str = "Val is a valid data type";

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
        Ok(*context.data::<Val>().expect(VALID_DATA).read().await)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// Sets a new value
    async fn set_value(&self, context: &Context<'_>, value: i32) -> Result<bool> {
        *context.data::<Val>().expect(VALID_DATA).write().await = value;
        Ok(true)
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

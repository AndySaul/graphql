use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;

pub struct Query;

#[Object]
impl Query {
    #[allow(clippy::unused_async)]
    async fn hello(&self) -> &'static str {
        "world"
    }
}

/// Creates a new GraphQL endpoint for poem server
#[must_use]
pub fn new() -> GraphQL<Schema<Query, EmptyMutation, EmptySubscription>> {
    GraphQL::new(Schema::build(Query, EmptyMutation, EmptySubscription).finish())
}

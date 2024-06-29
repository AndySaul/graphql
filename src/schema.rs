use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

pub fn new() -> GraphQL<Schema<Query, EmptyMutation, EmptySubscription>> {
    GraphQL::new(Schema::build(Query, EmptyMutation, EmptySubscription).finish())
}

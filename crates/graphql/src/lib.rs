use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[Object]
impl QueryRoot {
    pub async fn test(&self) -> &str {
        "This is a test query"
    }
}

pub fn build_schema() -> AppSchema {
    Schema::new(QueryRoot, EmptyMutation, EmptySubscription)
}

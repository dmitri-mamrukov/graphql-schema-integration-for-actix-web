# graphql-schema-integration-for-actix-web
GraphQL integration for Actix Web

## Testing

`cargo test -- --test-threads=1 --nocapture`

## Running

1. Start the GraphQL server via `cargo run`

2. `http://127.0.0.1:8080/graphiql`

3. Type, for instance, `{ allPosts { id title } }`.

See `test/integration_tests.rs` and `post_schema.graphql` for other sample queries.
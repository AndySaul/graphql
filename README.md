# GraphQL playground

Debug build & run:

```cargo run```

This will launch a [GraphiQL IDE](https://github.com/graphql/graphiql) at http://127.0.0.1:54321/.

You can specify a different port number on the command line:

```http://127.0.0.1:54321```

You can change log defaults by setting ```GRAPHQL_APP_LOG_LEVEL``` & ```GRAPHQL_APP_LOG_STYLE``` environment variables:

```GRAPHQL_APP_LOG_LEVEL="debug" cargo run```

## Clippy lints

Run all lints:

``` cargo clippy --all -- -D warnings```

Lint levels are modified in Cargo.toml ```[lints.clippy]``` block.

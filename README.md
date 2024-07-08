# GraphQL playground

This app creates a [GraphQL](https://graphql.org/) server with a simple API.

There is a single ```Int``` value that can be mutated. This value is synchronised so that multiple clients can query & mutate in parallel.

A real API would have much richer types, the server would handle GET and POST requests and user authentication, and changing the value would do something useful.

## Building

Debug build & run:

```cargo run```

This will launch a [GraphiQL IDE](https://github.com/graphql/graphiql) at http://127.0.0.1:54321/. This will show the API and allow clients to post queries and mutations.

You can specify a different port number on the command line:

```cargo run -- --port 55555```

You can change log defaults by setting ```GRAPHQL_APP_LOG_LEVEL``` & ```GRAPHQL_APP_LOG_STYLE``` environment variables:

```GRAPHQL_APP_LOG_LEVEL="debug" cargo run```

## GraphQL Queries

Launch the GraphQL IDE in the browser.

GraphQL shows only the results that are requested. You can set up a query to get all results:

```GraphQL
query GetAll {
  hello
  value
}
```

The results are shown as a JSON object:

```JSON
{
  "data": {
    "hello": "world",
    "value": 42
  }
}
```

Specific fields can also be requested:

```GraphQL
query GetValue {
  value
}
```
Result:

```JSON
{
  "data": {
    "value": 42
  }
}
```

## GraphQL mutation

The mutation requires a value in JSON format set in the Variables box:

```JSON
{
  "value": 23
}
```

Use the ```mutation``` keyword and reference the ```value``` variable:

```GraphQL
mutation SetValue($value: Int!) {
  setValue(value: $value) 
}
```
Result:

```JSON
{
  "data": {
    "setValue": true
  }
}
```


## Clippy lints

Run all lints:

``` cargo clippy --all -- -D warnings```

Lint levels are modified in Cargo.toml ```[lints.clippy]``` block.

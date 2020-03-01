## Routes

We want to support the following routes

```
parrotify/<message>/:background/:foreground
```

This can be a `GET` request.

The result is the parrotified version of the message.

## Testing endpoints

```bash
curl http://localhost:8088/parrotify/2
```

## TODO

- Use `serde` to parse the url params
- integrate `parrotify` library and use in route

### Post example
```bash
>curl --header "Content-Type: application/json" -X POST --data @data.json  http://localhost:8088/something/
```

Or using `httpie`

```bash
http POST localhost:8088/something/ id=hi name=bob
```

## Architecture

Parrotify will be a service. It could just be a crate we pull in.

### Routes
Take a look at [this](https://github.com/expressjs/express/tree/master/examples/mvc)
as a starting point.
Or [this](https://dev.to/werner/practical-rust-web-development-api-rest-29g1).


## Security

https://expressjs.com/en/advanced/best-practice-security.html

## Running in dev mode

Install the required modules and start the monitoring

```bash
cargo install systemfd cargo-watch
systemfd --no-pid -s http::8088 -- cargo watch -x run
```

Without dev mode, just use `cargo run`
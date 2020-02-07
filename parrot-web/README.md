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

- `curl http://localhost:8088/parrotify` is not
the same as `>curl http://localhost:8088/parrotify/`. Why?

### Post example
```bash
>curl --header "Content-Type: application/json" -X POST --data @data.json  http://localhost:8088/something/
```

Or using `httpie`

```bash
http POST localhost:8088/something/ id=hi name=bob
```

# QUIC Iterators

QUIC Iterators is an experimental project to send iterable data over a QUIC
connection. This allows data to be processed on the server before the entire
set has arrived, providing a more responsive application.

## Development

The current implementation uses a client/server model, with the client
implementation in `src/client.rs` and the server implementation in
`src/server.rs`.

To test, you should run the server prior to the client. The server can be run
with `cargo run --bin server` and the client with `cargo run --bin client`. The
client will connect to the server and send it the message `"Hello"`, to which
the server will respond with `"World"`.

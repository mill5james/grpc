# Simple gRPC example using Rust and ASP.NET Core

This is a very basic example of implementing both a gRPC client and server using Rust and .NET.

A client implemented in one language can talk to a server implemented in either language.

## Directories in this repository

- [Common](./Common/)
  - Contains the elements used by both client and server in all languages
- [Client](./Client/)
  - The gRPC .NET Console client
- [Server](./Server/)
  - The ASP.NET Core gRPC server
- [rust](./rust/)
  - The rust implementation of both the client and server

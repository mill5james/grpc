# Rust gRPC implementation with Tonic

This is an implementation of a gRPC service and client described in the [Example.proto](../Common/Example.proto) file using Rust leveraging the [tonic](https://docs.rs/tonic/latest/tonic/) crate. There is good information at the GitHub repository for [Tonic](https://github.com/hyperium/tonic) including [examples](https://github.com/hyperium/tonic/tree/master/examples) and additional documentation.

## Prerequisites

### Protocol Buffer Compiler

In order to build `tonic`, you need the Protocol Buffers compiler - `protoc`.

#### Windows

You can easily install this on Windows using the [Chocolatey](https://chocolatey.org/) package [protoc](https://community.chocolatey.org/packages/protoc)

```text
choco install protoc
```

#### MacOS

On MacOS, use [Homebrew](https://brew.sh/) for easy installation

``` bash
brew install protobuf
```

#### Other OS

Follow the instructions from the gRPC website [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).

### HTTPS certificate

The Rust client and server need a certificate to validate the HTTPS connection. We borrow the .NET self-signed certificate for HTTPS to use in our client and server. To do this we export the certificate to a `certificate.pfx` with the password ***password*** then use the [openssl](https://community.chocolatey.org/packages/openssl) command to extract the `certificate.pem` and `certificate.key` files, entering in the password when prompted for a pass phrase.

``` text
C:\grpc\rust> dotnet dev-certs https --export-path ./certificate.pfx --password password --trust --format pfx
Trusting the HTTPS development certificate was requested. A confirmation prompt will be displayed if the certificate was not previously trusted. Click yes on the prompt to trust the certificate.
A valid HTTPS certificate is already present.
C:\grpc\rust> openssl x509 -in ./certificate.pfx -out ./certificate.pem -clcerts -nokeys
Enter pass phrase for PKCS12 import pass phrase:
C:\grpc\rust> openssl rsa -in ./certificate.pfx -out ./certificate.key
Enter pass phrase for PKCS12 import pass phrase:
writing RSA key
```

You should now have a `certificate.pem` and `certificate.key` file in your `grpc\rust` directory.

## Building

To build the `client` and `server`, we just need to run the `cargo build` command

``` text
C:\grpc\rust> cargo build
   Compiling proc-macro2 v1.0.46
    .
    .
    .
   Compiling tonic v0.8.2
    Finished dev [unoptimized + debuginfo] target(s) in 1m 32s
```

## Running the Server

To run the server we use `cargo run` with the `server` bin target

``` text
C:\grpc\rust> cargo run --bin server
   Compiling grpc_example v0.1.0 (C:\grpc\rust)
    Finished dev [unoptimized + debuginfo] target(s) in 8.97s
     Running `target\debug\server.exe`
```

## Running the Client

To run the server we use `cargo run` with the `client` bin target

``` text
C:\grpc\rust> cargo run --bin server
   Compiling grpc_example v0.1.0 (C:\grpc\rust)
    Finished dev [unoptimized + debuginfo] target(s) in 3.79s
     Running `target\debug\client.exe`
Response: ServerResponse { message: "Hello World" }
Press enter to continue
.
.
.

```

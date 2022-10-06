# ASP.NET Core gRPC server

This is a ASP.NET Core server that exposes a gRPC service defined by the [Example.proto](../Common/Example.proto). This uses the [gRPC on .NET](https://learn.microsoft.com/en-us/aspnet/core/grpc/) tooling in ASP.NET Core available in .NET Core 3.x or later.

The server can be used by both the .NET and Rust clients.

## Prerequisites

Since the server exposes an HTTPS endpoint on localhost, the server requires the self-signed HTTPS development certificate to be installed upon the machine. This can be done easily using the [dotnet dev-certs](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-dev-certs) command.

``` text
C:\grpc\Server> dotnet dev-certs https
The HTTPS developer certificate was generated successfully.
```

If the command finds a development certificate, it displays a message like the following example:

``` text
C:\grpc\Server> dotnet dev-certs https
A valid HTTPS certificate is already present.
```

## Building the Server

Use the `dotnet build` command in the `.\Server` directory to build

``` text
C:\grpc\Server> dotnet build
MSBuild version 17.3.1+2badb37d1 for .NET
  Determining projects to restore...
  All projects are up-to-date for restore.
  Common -> C:\grpc\Common\bin\Debug\netstandard2.1\Common.dll
  Server -> C:\grpc\Server\bin\Debug\net6.0\Server.dll

Build succeeded.
    0 Warning(s)
    0 Error(s)

Time Elapsed 00:00:04.24
```

## Running the Server

Use the `dotnet run` command in the `.\Server` directory to run.

``` text
C:\grpc\Client> dotnet run
info: Microsoft.Hosting.Lifetime[14]
      Now listening on: https://localhost:5001
info: Microsoft.Hosting.Lifetime[0]
      Application started. Press Ctrl+C to shut down.
info: Microsoft.Hosting.Lifetime[0]
      Hosting environment: Development
info: Microsoft.Hosting.Lifetime[0]
      Content root path: C:\grpc\Server\
.
.
.

```

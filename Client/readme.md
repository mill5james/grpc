# .NET gRPC Client

This is a simple .NET console client that exercises all of the endpoints on the gRPC server. This uses the [gRPC on .NET](https://learn.microsoft.com/en-us/aspnet/core/grpc/) tooling in ASP.NET Core since .NET Core 3.x or later.

The client can connect to the gRPC server from .NET or Rust.

## Building the Client

Use the `dotnet build` command in the `.\Client` directory to build

``` console
C:\grpc\Client> dotnet build
MSBuild version 17.3.1+2badb37d1 for .NET
  Determining projects to restore...
  Restored C:\grpc\Common\Common.csproj (in 282 ms).
  Restored C:\grpc\Client\Client.csproj (in 282 ms).
  Common -> C:\grpc\Common\bin\Debug\netstandard2.1\Common.dll
  Client -> C:\grpc\Client\bin\Debug\net6.0\Client.dll

Build succeeded.
    0 Warning(s)
    0 Error(s)

Time Elapsed 00:00:03.69
```

## Running the Client

Use the `dotnet run` command in the `.\Client` directory to run. Make sure you run either the [.NET Server](../Server/readme.md) or [Rust server](../rust/readme.md) before starting the client.

``` console
C:\grpc\Client> dotnet run
Response: Hello world
Press enter to continue...
.
.
.

```

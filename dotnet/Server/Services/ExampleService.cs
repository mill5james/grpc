using Grpc.Core;
using System.Reactive.Linq;
using GrpcExample;

namespace GrpcServer;

public class ExampleService : Example.ExampleBase
{
    private readonly ILogger<ExampleService> logger;
    public ExampleService(ILogger<ExampleService> logger)
    {
        this.logger = logger;
    }

    public override Task<ServerResponse> Simple(ClientRequest request, ServerCallContext context)
    {
        return Task.FromResult(new ServerResponse
        {
            Message = "Hello " + request.Message
        });
    }

    public override async Task<ServerResponse> ClientStream(IAsyncStreamReader<ClientStreamMsg> requestStream, ServerCallContext context)
    {
        int count = 0;
        await foreach (var msg in requestStream.ReadAllAsync())
        {
            logger.LogInformation($"Got ClientStreamMsg {{ Message = \"{msg.Message}\" }}");
            count++;
        }
        return new ServerResponse{ Message = $"Received {count} messages"};
    }

    public override async Task ServerStream(ClientRequest request, IServerStreamWriter<ServerStreamMsg> responseStream, ServerCallContext context)
    {
        int sendCount = 10; 
        int.TryParse(request.Message, out sendCount);
        for (int i = 0; i < sendCount; i++) 
        { 
            logger.LogInformation("Sending {0}", i);
            await responseStream.WriteAsync(new ServerStreamMsg { Message = $"Message {i}"});
            await Task.Delay(1000);
        }
    }

    public override Task BiDirStream(IAsyncStreamReader<ClientStreamMsg> requestStream, IServerStreamWriter<ServerStreamMsg> responseStream, ServerCallContext context)
    {
        return base.BiDirStream(requestStream, responseStream, context);
    }
}

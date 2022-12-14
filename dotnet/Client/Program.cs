#nullable enable

using Grpc.Net.Client;
using Grpc.Core;
using GrpcExample;

namespace GrpcClient;

public class Program
{
    static async Task Main(string[] args)
    {
        var channel = GrpcChannel.ForAddress("https://localhost:5001");
        var client = new Example.ExampleClient(channel);

        //Call unary method
        var reply = await client.SimpleAsync(new ClientRequest { Message = "world" });
        Console.WriteLine("Response: " + reply.Message);
        Console.WriteLine("Press enter to continue...");
        Console.ReadLine();

        using (var tokenSource = new CancellationTokenSource())
        {
            //Call client streaming method
            using (var clientStream = client.ClientStream(cancellationToken: tokenSource.Token))
            {
                var writer = clientStream.RequestStream;
                for (int i = 0; i < 10; i++)
                {
                    Console.WriteLine($"Sending ClientStreamMsg {{ Message = \"Message {i}\" }}");
                    await writer.WriteAsync(new ClientStreamMsg { Message = $"Message {i}" });
                    await Task.Delay(1000);
                }
                await writer.CompleteAsync();
                Console.WriteLine("=== Client Stream Complete ===");
                var response = await clientStream.ResponseAsync;
                Console.WriteLine($"Response: ServerResponse {response}");
                Console.WriteLine("Press enter to continue...");
                Console.ReadLine();
            }

            //Call server streaming method
            using (AsyncServerStreamingCall<ServerStreamMsg> asyncServerStreamingCall = client.ServerStream(new ClientRequest { Message = "21" }, cancellationToken: tokenSource.Token))
            {
                await foreach (var msg in asyncServerStreamingCall.ResponseStream.ReadAllAsync(tokenSource.Token))
                {
                    Console.WriteLine($"Received ServerStreamMsg {msg}");
                }
            }
            Console.WriteLine("=== Server Stream Complete ===");
            Console.WriteLine("Press enter to continue...");
            Console.ReadLine();
        }
    }
}

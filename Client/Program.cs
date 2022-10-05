//Added NuGet Package Grpc.Net.Client
#nullable enable

using Grpc.Net.Client;
using Grpc.Core;
using GrpcExample;

namespace GrpcClient;

public class Program
{
    private static Random rng = new Random();

    static async Task Main(string[] args)
    {
        string?[] errors = {};
        var error = string.Join(',', errors);
        var channel = GrpcChannel.ForAddress("https://localhost:5001");
        var client = new Example.ExampleClient(channel);

        var id = rng.Next();

        //Call unary method
        var reply = await client.SimpleAsync(new ClientRequest { Message = "world" });
        Console.WriteLine("Response: " + reply.Message);
        Console.WriteLine("Press any key to continue...");
        Console.Read();
        using (var tokenSource = new CancellationTokenSource())
        {
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
                Console.WriteLine("Press any key to continue...");
                Console.Read();
            }

            using (AsyncServerStreamingCall<ServerStreamMsg> asyncServerStreamingCall = client.ServerStream(new ClientRequest { Message = "21" }, cancellationToken: tokenSource.Token))
            {
                await foreach (var msg in asyncServerStreamingCall.ResponseStream.ReadAllAsync(tokenSource.Token))
                {
                    Console.WriteLine($"Received ServerStreamMsg {msg}");
                }
            }
            Console.WriteLine("=== Server Stream Complete ===");

            Console.Read();
        }
    }

    // static async Task ServerEvents(int id, Service.ServiceClient client, CancellationToken token)
    // {
    //     ClientIdentifer clientIdentifer = new ClientIdentifer { Id = id };

    //     clientIdentifer.Subscriptions.AddRange(Enumerable.Range(0, rng.Next(2,20)).Select(_ => rng.Next(20)).Distinct());
    //     // clientIdentifer.Subscriptions.AddRange(Enumerable.Range(0, 20));

    //     Console.WriteLine($"ID:\t{clientIdentifer.Id}");
    //     Console.WriteLine($"Subs:\t{string.Join(",", clientIdentifer.Subscriptions.OrderBy(_ => _))}");

    //     using (AsyncServerStreamingCall<ServerMessage> keepAliveCall = client.ServerEvents(clientIdentifer, cancellationToken: token))
    //     {
    //         await foreach (var msg in keepAliveCall.ResponseStream.ReadAllAsync(token))
    //         {
    //             if (msg.Id == -1) break;
    //             Console.WriteLine($"{msg.Id} - {msg.Message}");
    //         }
    //     }
    //     Console.WriteLine("=== Stream Complete ===");
    // }
}



using System;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace GrpcServer;

public class GeneratedEventArgs : EventArgs
{
    public GeneratedEventArgs(int id, DateTime timeStamp)
    {
        Id = id;
        TimeStamp = timeStamp;
    }

    public int Id { get; set; }
    public DateTime TimeStamp { get; set; }
}

public interface IEventGeneratorService {
    event EventHandler<GeneratedEventArgs> MessageEvent;

}
public class EventGeneratorService : BackgroundService, IEventGeneratorService
{
    private readonly ILogger<EventGeneratorService> _logger;
    public EventGeneratorService(ILogger<EventGeneratorService> logger)
    {
        _logger = logger;
    }

    public event EventHandler<GeneratedEventArgs>? MessageEvent;

    protected override async Task ExecuteAsync(CancellationToken token)
    {
        var rng = new Random();
        while (!token.IsCancellationRequested)
        {
            var id = rng.Next(20);
            var timeStamp = DateTime.Now;
            await Task.Run(() => MessageEvent?.Invoke(this, new GeneratedEventArgs(id, timeStamp)), token);
            await Task.Delay(1000, token);
        }
    }
}
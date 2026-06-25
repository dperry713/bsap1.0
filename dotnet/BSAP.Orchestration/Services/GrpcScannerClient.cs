using BSAP.Grpc;
using Grpc.Net.Client;
using Microsoft.Extensions.Logging;

namespace BSAP.Orchestration.Services;

public class GrpcScannerClient : IScannerClient
{
    private readonly ScannerService.ScannerServiceClient _client;
    private readonly ILogger<GrpcScannerClient> _logger;

    public GrpcScannerClient(GrpcChannel channel, ILogger<GrpcScannerClient> logger)
    {
        _client = new ScannerService.ScannerServiceClient(channel);
        _logger = logger;
    }

    public async Task<ScanResponse> PerformScanAsync(ScanRequest request, CancellationToken ct = default)
    {
        _logger.LogInformation("Sending scan request: {ScanType} for {Target}", request.ScanType, request.Target);
        return await _client.PerformScanAsync(request, cancellationToken: ct);
    }

    public async Task<BatchScanResponse> PerformBatchScanAsync(BatchScanRequest request, CancellationToken ct = default)
    {
        return await _client.PerformBatchScanAsync(request, cancellationToken: ct);
    }
}

public interface IScannerClient
{
    Task<ScanResponse> PerformScanAsync(ScanRequest request, CancellationToken ct = default);
    Task<BatchScanResponse> PerformBatchScanAsync(BatchScanRequest request, CancellationToken ct = default);
}
using BSAP.Common.Models;
using BSAP.Grpc;
using BSAP.Orchestration.RiskEngine;
using BSAP.Orchestration.Services;
using Hangfire;
using Microsoft.AspNetCore.Mvc;

namespace BSAP.Orchestration.Controllers;

[ApiController]
[Route("api/[controller]")]
public class ScanController : ControllerBase
{
    private readonly IScannerClient _scannerClient;
    private readonly RiskEngine _riskEngine;
    private readonly IBackgroundJobClient _jobClient;

    public ScanController(IScannerClient scannerClient, RiskEngine riskEngine, IBackgroundJobClient jobClient)
    {
        _scannerClient = scannerClient;
        _riskEngine = riskEngine;
        _jobClient = jobClient;
    }

    [HttpPost]
    public async Task<IActionResult> StartScan([FromBody] ScanRequestDto request)
    {
        var grpcRequest = new ScanRequest
        {
            ScanId = request.ScanId ?? Guid.NewGuid().ToString(),
            ScanType = request.ScanType,
            Target = request.Target,
            TenantId = request.TenantId ?? "default"
        };

        // Fire-and-forget background job
        var jobId = _jobClient.Enqueue(() => ProcessScan(grpcRequest));
        
        return Accepted(new { JobId = jobId, ScanId = grpcRequest.ScanId });
    }

    [ApiExplorerSettings(IgnoreApi = true)]
    public async Task ProcessScan(ScanRequest grpcRequest)
    {
        var response = await _scannerClient.PerformScanAsync(grpcRequest);
        
        var findings = response.Findings.Select(f => new Finding
        {
            Id = f.Id,
            Severity = f.Severity,
            Cwe = f.Cwe,
            Type = f.Type,
            Description = f.Description,
            Evidence = f.Evidence,
            Confidence = f.Confidence,
            Remediation = f.Remediation
        }).ToList();

        var riskConfig = new RiskConfig { ScanId = grpcRequest.ScanId };
        var riskAssessment = _riskEngine.CalculateRisk(findings, riskConfig);

        // Persist to DB
        // TODO: Inject DbContext for full persistence
    }
}

public class ScanRequestDto
{
    public string? ScanId { get; set; }
    public string ScanType { get; set; } = string.Empty;
    public string Target { get; set; } = string.Empty;
    public string? TenantId { get; set; }
}
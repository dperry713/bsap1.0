using BSAP.Common.Models;
using System.Text.Json;

namespace BSAP.Orchestration.RiskEngine;

public class RiskEngine
{
    private readonly Dictionary<string, double> _severityWeights = new()
    {
        { "critical", 10.0 },
        { "high", 7.5 },
        { "medium", 5.0 },
        { "low", 2.5 },
        { "informational", 0.5 }
    };

    public RiskAssessment CalculateRisk(IEnumerable<Finding> findings, RiskConfig config)
    {
        if (!findings.Any())
            return new RiskAssessment { CvssScore = 0.0, OverallRisk = "LOW" };

        double totalScore = 0;
        int criticalCount = 0, highCount = 0;

        foreach (var finding in findings)
        {
            var weight = _severityWeights.GetValueOrDefault(finding.Severity.ToLower(), 1.0);
            totalScore += weight * finding.Confidence;

            if (finding.Severity.ToLower() == "critical") criticalCount++;
            if (finding.Severity.ToLower() == "high") highCount++;
        }

        double cvssBase = Math.Min(10.0, totalScore / findings.Count() * 1.2);

        var assessment = new RiskAssessment
        {
            ScanId = config.ScanId,
            CvssScore = Math.Round(cvssBase, 1),
            OwaspCategory = MapToOwasp(findings),
            MitreTactic = MapToMitre(findings),
            OverallRisk = DetermineOverallRisk(cvssBase, criticalCount, highCount),
            ComplianceMappings = MapCompliance(findings)
        };

        return assessment;
    }

    private string MapToOwasp(IEnumerable<Finding> findings) => findings.Any(f => f.Cwe.StartsWith("CWE-79")) ? "A03:2021" : "A01:2021";
    private string MapToMitre(IEnumerable<Finding> findings) => "TA0006"; // Credential Access example
    private string DetermineOverallRisk(double score, int crit, int high)
    {
        if (crit > 0 || score >= 9.0) return "CRITICAL";
        if (high > 2 || score >= 7.0) return "HIGH";
        return score >= 4.0 ? "MEDIUM" : "LOW";
    }

    private List<string> MapCompliance(IEnumerable<Finding> findings) => new() { "PCI-DSS", "SOC2", "NIST-800-53" };
}
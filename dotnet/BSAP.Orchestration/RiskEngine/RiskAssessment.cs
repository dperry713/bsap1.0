namespace BSAP.Orchestration.RiskEngine;

public class RiskAssessment
{
    public string ScanId { get; set; } = string.Empty;
    public double CvssScore { get; set; }
    public string OwaspCategory { get; set; } = string.Empty;
    public string MitreTactic { get; set; } = string.Empty;
    public string OverallRisk { get; set; } = string.Empty;
    public List<string> ComplianceMappings { get; set; } = new();
}

public class RiskConfig
{
    public string ScanId { get; set; } = string.Empty;
    public Dictionary<string, string> Options { get; set; } = new();
}
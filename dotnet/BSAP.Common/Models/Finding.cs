using System.Text.Json.Serialization;

namespace BSAP.Common.Models;

public class Finding
{
    public string Id { get; set; } = string.Empty;
    public string Severity { get; set; } = string.Empty;
    public string Cwe { get; set; } = string.Empty;
    public string Type { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
    public string Evidence { get; set; } = string.Empty;
    public double Confidence { get; set; }
    public string Remediation { get; set; } = string.Empty;
    public Dictionary<string, string> Metadata { get; set; } = new();
}
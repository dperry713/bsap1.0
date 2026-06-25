use scanner_core::Finding;

pub mod prototype_pollution;

use scanner_core::Finding;

pub fn analyze_api_security(code: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    
    // Delegate to prototype pollution
    findings.extend(prototype_pollution::analyze_prototype_pollution(code));
    
    // Add more API checks: JWT, CORS, etc. patterns
    if code.contains("jwt.sign") && code.contains("none") {
        findings.push(Finding {
            id: "API-001".to_string(),
            severity: "critical".to_string(),
            cwe: Some("CWE-345".to_string()),
            description: "JWT 'none' algorithm usage - critical auth bypass risk".to_string(),
            evidence: "Algorithm: none detected".to_string(),
            confidence: 0.95,
            remediation: Some("Enforce RS256/ES256 and validate alg header".to_string()),
        });
    }
    
    findings
}

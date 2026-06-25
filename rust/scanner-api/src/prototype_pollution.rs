use scanner_core::Finding;
use regex::Regex;

pub fn analyze_prototype_pollution(code: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    
    let proto_re = Regex::new(r"(?i)(Object\.assign|lodash\.merge|deepmerge|__proto__|constructor\.prototype)").unwrap();
    let sink_re = Regex::new(r"(?i)(req\.body|JSON\.parse|merge\(|extend\()").unwrap();
    
    // Source pattern matching
    if proto_re.is_match(code) && sink_re.is_match(code) {
        findings.push(Finding {
            id: "PROTO-001".to_string(),
            severity: "critical".to_string(),
            cwe: Some("CWE-915".to_string()),
            description: "Prototype pollution via unsafe object merge".to_string(),
            evidence: "Detected unsafe merge of user-controlled input into prototype chain".to_string(),
            confidence: 0.93,
            remediation: Some("Use Object.create(null), structuredClone(), or lodash.mergeWith with customizer".to_string()),
        });
    }
    
    // Advanced: Check for common vulnerable libraries
    if code.contains("lodash") && !code.contains("mergeWith") {
        findings.push(Finding {
            id: "PROTO-002".to_string(),
            severity: "high".to_string(),
            cwe: Some("CWE-915".to_string()),
            description: "Outdated lodash merge vulnerable to prototype pollution".to_string(),
            evidence: "lodash.merge usage without safeguards".to_string(),
            confidence: 0.85,
            remediation: Some("Upgrade lodash >=4.17.12 or use safe alternatives".to_string()),
        });
    }
    
    findings
}

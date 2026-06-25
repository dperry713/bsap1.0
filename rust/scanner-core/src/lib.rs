pub mod binary;
pub mod memory;
pub mod malware;
pub mod api;
pub mod container;
pub mod dependency;
pub mod runtime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Finding {
    pub id: String,
    pub severity: String,
    pub cwe: Option<String>,
    pub r#type: String,  // Added for better mapping
    pub description: String,
    pub evidence: String,
    pub confidence: f64,
    pub remediation: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub scan_id: String,
    pub findings: Vec<Finding>,
    pub risk_score: f64,
}

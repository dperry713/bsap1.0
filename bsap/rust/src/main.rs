use scanner_memory::analyze_memory;
use scanner_malware::scan_malware;
use scanner_api::analyze_api_security;
use scanner_container::analyze_container;
use scanner_runtime::analyze_runtime;
use scanner_dependency::analyze_dependencies;
use scanner_core::Finding;
use std::env;

fn main() {
    println!("🚀 BSAP Rust Scanner Engine - Production Mode");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bsap <scan_type> <target>");
        return;
    }

    let scan_type = &args[1];
    let target = if args.len() > 2 { &args[2] } else { "test_target" };

    let findings: Vec<Finding> = match scan_type.as_str() {
        "memory" => analyze_memory(target).unwrap_or_default(),
        "malware" => scan_malware(target).unwrap_or_default(),
        "api" => analyze_api_security(target),
        "container" => analyze_container(target).unwrap_or_default(),
        "runtime" => analyze_runtime(target, "api").unwrap_or_default(),
        "dependency" => analyze_dependencies(target),
        _ => vec![],
    };

    println!("Scan complete. Findings: {}", findings.len());
    for f in findings {
        println!("[{}] {} - {} (Confidence: {:.2})", f.severity.to_uppercase(), f.id, f.description, f.confidence);
    }
}
pub mod bsap {
    tonic::include_proto!("bsap");
}

use bsap::scanner_service_server::ScannerService;
use bsap::{ScanRequest, ScanResponse, Finding as GrpcFinding, Empty, StatusResponse};
use tonic::{Request, Response, Status};
use scanner_core::Finding;
use std::collections::HashMap;

#[derive(Default)]
pub struct BsapScannerServer;

#[tonic::async_trait]
impl ScannerService for BsapScannerServer {
    async fn perform_scan(
        &self,
        request: Request<ScanRequest>,
    ) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = self.dispatch_scan(&req).await?;
        
        let grpc_findings: Vec<GrpcFinding> = findings.into_iter().map(|f| GrpcFinding {
            id: f.id,
            severity: f.severity,
            cwe: f.cwe.unwrap_or_default(),
            r#type: "general".to_string(), // map from scanner type
            description: f.description,
            evidence: f.evidence,
            confidence: f.confidence,
            remediation: f.remediation.unwrap_or_default(),
            metadata: HashMap::new(),
        }).collect();

        let response = ScanResponse {
            scan_id: req.scan_id,
            findings: grpc_findings,
            risk_score: 0.0, // .NET risk engine will compute
            status: "completed".to_string(),
            message: "Scan completed successfully".to_string(),
            duration_ms: 1500, // placeholder
        };

        Ok(Response::new(response))
    }

    async fn perform_batch_scan(
        &self,
        _request: Request<bsap::BatchScanRequest>,
    ) -> Result<Response<bsap::BatchScanResponse>, Status> {
        // TODO: Parallel dispatch with rayon/tokio
        unimplemented!("Batch scanning - high throughput mode")
    }

    async fn scan_memory(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = scanner_memory::analyze_memory(&req.target)
            .map_err(|e| Status::internal(e.to_string()))?;
        // Convert and return...
        self.convert_to_response(req.scan_id, findings)
    }

    // Similar impls for other scanners...
    async fn scan_malware(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        // Implement similarly using scanner_malware
        unimplemented!()
    }

    async fn scan_api(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = scanner_api::analyze_api_security(&req.target);
        self.convert_to_response(req.scan_id, findings)
    }

    async fn scan_container(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = scanner_container::analyze_container(&req.target)
            .map_err(|e| Status::internal(e.to_string()))?;
        self.convert_to_response(req.scan_id, findings)
    }

    async fn scan_runtime(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = scanner_runtime::analyze_runtime(&req.target, "api")
            .map_err(|e| Status::internal(e.to_string()))?;
        self.convert_to_response(req.scan_id, findings)
    }

    async fn scan_dependencies(&self, request: Request<ScanRequest>) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let findings = scanner_dependency::analyze_dependencies(&req.target);
        self.convert_to_response(req.scan_id, findings)
    }

    async fn get_scanner_status(&self, _request: Request<Empty>) -> Result<Response<StatusResponse>, Status> {
        let mut capabilities = HashMap::new();
        capabilities.insert("memory".to_string(), "true".to_string());
        capabilities.insert("malware".to_string(), "true".to_string());
        // ... add others

        let response = StatusResponse {
            status: "healthy".to_string(),
            capabilities,
            version: 1,
        };
        Ok(Response::new(response))
    }

    fn convert_to_response(&self, scan_id: String, findings: Vec<Finding>) -> Result<Response<ScanResponse>, Status> {
        let grpc_findings: Vec<GrpcFinding> = findings.into_iter().map(|f| GrpcFinding {
            id: f.id,
            severity: f.severity,
            cwe: f.cwe.unwrap_or_default(),
            r#type: "scanner".to_string(),
            description: f.description,
            evidence: f.evidence,
            confidence: f.confidence,
            remediation: f.remediation.unwrap_or_default(),
            metadata: HashMap::new(),
        }).collect();

        let response = ScanResponse {
            scan_id,
            findings: grpc_findings,
            risk_score: 0.0,
            status: "completed".to_string(),
            message: format!("Scan completed with {} findings", findings.len()),
            duration_ms: 1200,
        };
        Ok(Response::new(response))
    }
}

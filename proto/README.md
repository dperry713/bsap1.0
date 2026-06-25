# BSAP gRPC Protobuf Contracts

## Overview
Production-grade gRPC contracts for .NET (C#) Orchestration ↔ Rust Scanner Engine communication.

## Generation Instructions

### For Rust (Tonic)
```bash
cargo install tonic-build
# Add to Cargo.toml in scanner-core or a new grpc crate:
# tonic = "0.12"
# prost = "0.13"

# Build with build.rs
```

### For C# (.NET 9)
```bash
dotnet tool install --global Grpc.Tools
# Use Grpc.Tools in .csproj for code generation
```

### Key Features
- Supports single + batch scanning
- Rich Finding model aligned with scanner-core
- Multi-tenancy (tenant_id)
- Extensible options/metadata
- Health checking
- Ready for mTLS + JWT auth in production

## Integration Flow
1. .NET sends ScanRequest via gRPC
2. Rust performs analysis (memory, container, etc.)
3. Returns ScanResponse with findings
4. .NET aggregates, applies Risk Engine (CVSS/OWASP), stores in PostgreSQL
```

Now generate the build script for Rust.
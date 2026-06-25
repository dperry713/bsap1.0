# BSAP Architecture

## Overview
Backend Security Analysis Platform (BSAP) is a multi-language, modular platform for comprehensive backend security analysis.

## Layers
1. **Rust Scanning Engine**: High-performance core for binary/memory/malware analysis.
2. **C# Orchestration**: Job scheduling, API, risk scoring.
3. **Node.js Dashboard**: Real-time UI with Next.js.

## Data Flow
Scan Request -> C# Orchestrator -> Rust Scanner -> Results -> PostgreSQL/Redis -> Dashboard via WebSockets.

## Key Components
- gRPC for Rust-C# communication.
- REST/gRPC APIs.
- Containerized with Docker/K8s.

## Scalability
- Horizontal scaling with Kubernetes.
- Rayon for parallel Rust scans.
- Redis for caching, OpenSearch for search.

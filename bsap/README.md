# Backend Security Analysis Platform (BSAP)

**Enterprise-grade vulnerability analysis platform** for backend systems, APIs, containers, binaries, and cloud infrastructure.

Built with:
- **Rust** - High-performance scanning engine
- **C# (.NET 9)** - Orchestration, risk scoring, APIs
- **Next.js** - Real-time dashboard

---

## ✨ Features

### Core Scanners (Rust)
- **Memory Corruption** - Buffer overflows, UAF, heap corruption via Capstone disassembly
- **Malware Analysis** - YARA-X, behavioral heuristics, persistence detection
- **API Security** - Prototype pollution, JWT weaknesses, injection patterns
- **Container Security** - Docker/K8s RBAC, privileged containers, secrets leakage
- **Runtime Analysis** - DAST injection testing, unsafe deserialization
- **Dependency Analysis** - SBOM, CVE detection, supply chain risks

### Orchestration (.NET)
- gRPC integration with Rust scanners
- Hangfire background jobs
- CVSS + OWASP + MITRE ATT&CK Risk Engine
- PostgreSQL persistence

### Dashboard (Next.js)
- Real-time WebSocket updates
- Executive summaries, findings explorer, risk heatmaps
- Live scan progress

### Production Ready
- Horizontal scaling (Kubernetes)
- mTLS, RBAC, audit logging
- Observability (OpenTelemetry)
- Multi-tenancy

---

## 📁 Project Structure

```
bsap/
├── rust/                  # Scanner engine + gRPC server
├── dotnet/                # Orchestration + API
├── nodejs/dashboard/      # Next.js UI
├── proto/                 # gRPC contracts
├── infra/                 # Docker & K8s
├── schemas/               # DB schemas
├── docs/                  # Documentation
└── shared/                # Common models
```

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Rust (cargo)
- .NET 9 SDK
- Node.js 20+

### Development Setup

1. **Clone / Extract** the project
2. **Start infrastructure**
   ```bash
   cd bsap
   docker compose -f infra/docker/docker-compose.dev.yml up -d
   ```

3. **Build Rust gRPC Server**
   ```bash
   cd rust
   cargo build --release
   cargo run -p scanner-grpc
   ```

4. **Run .NET Orchestration**
   ```bash
   cd dotnet/BSAP.Orchestration
   dotnet run
   ```

5. **Launch Dashboard**
   ```bash
   cd nodejs/dashboard
   npm install
   npm run dev
   ```

6. **Access**
   - Dashboard: http://localhost:3000
   - API: http://localhost:8080
   - Rust gRPC: localhost:50051

### Production Deployment
See `docs/DEPLOYMENT.md`

---

## 🧪 Testing

```bash
# Rust
cd rust && cargo test

# .NET
cd dotnet && dotnet test

# Node.js
cd nodejs/dashboard && npm test
```

## 📊 Architecture Overview

- **Rust** → High perf binary/memory/malware analysis
- **gRPC** → Efficient communication
- **.NET** → Job orchestration + Risk scoring
- **Next.js** → Real-time UI

## 🔐 Security

- All communications use mTLS (prod)
- JWT + MFA support
- Secrets management ready (Vault)
- Sandboxed scanners

## 📄 License
Internal Enterprise Use / Custom License

---

**For full deployment guide, see `docs/DEPLOYMENT.md`**

Contributions, extensions, and AI integration welcome!
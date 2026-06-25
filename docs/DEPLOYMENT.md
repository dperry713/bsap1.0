# BSAP Production Deployment Guide

## Overview
This document provides comprehensive instructions for deploying **Backend Security Analysis Platform (BSAP)** in production environments.

## 🏗️ Architecture Components

- **Rust Scanner Service** (gRPC)
- **.NET Orchestration & API**
- **Next.js Dashboard**
- **PostgreSQL** + **Redis**
- **MinIO** (object storage)
- **OpenSearch** (search)

## Prerequisites

- Kubernetes cluster (EKS/GKE/AKS) or Docker Swarm
- Helm 3+
- Cert-Manager for TLS
- PostgreSQL operator or managed DB
- Redis + MinIO

## 1. Infrastructure Setup

### Docker Compose (Staging / Small Prod)

```bash
cd /path/to/bsap

# Copy env
cp .env.example .env
# Edit secrets, DB passwords, etc.

docker compose -f infra/docker/docker-compose.prod.yml up -d --build
```

### Kubernetes (Recommended Production)

```bash
# Install dependencies
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

# Deploy dependencies
helm install postgres bitnami/postgresql -f infra/k8s/postgres-values.yaml
helm install redis bitnami/redis
helm install minio bitnami/minio

# Deploy BSAP
cd infra/k8s
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secrets.yaml   # Use sealed-secrets in prod

# Deploy services
kubectl apply -f rust-scanner-deployment.yaml
kubectl apply -f dotnet-orchestration.yaml
kubectl apply -f nodejs-dashboard.yaml
```

## 2. Configuration

**Key Environment Variables** (`.env`):
- `RUST_SCANNER_URL=bsap-rust-scanner:50051`
- `DATABASE_URL=postgres://...`
- `REDIS_URL=redis://...`
- `JWT_SECRET=...` (strong random)
- `MINIO_ENDPOINT=...`

## 3. Security Hardening

- Enable mTLS between services
- Configure NetworkPolicies
- Enable PodSecurityPolicies / OPA Gatekeeper
- Rotate secrets regularly
- Enable audit logging

## 4. Scaling & High Availability

- Horizontal Pod Autoscaler (HPA) for Rust and .NET services
- Redis Cluster for queue
- PostgreSQL with replicas
- Multiple Rust scanner pods for parallel scans

## 5. Monitoring & Observability

- Prometheus + Grafana dashboards (included in infra/monitoring)
- OpenTelemetry traces
- Loki for logs
- Alerting on high risk findings

## 6. Backup & Disaster Recovery

- Daily DB backups
- MinIO versioning
- GitOps with ArgoCD (recommended)

## 7. First-Time Setup

```bash
# Run migrations
dotnet ef database update   # or via job

# Seed initial data
# Create admin user via API
```

## 8. Troubleshooting

- Check logs: `kubectl logs -f deployment/bsap-rust`
- gRPC connectivity: `grpcurl -plaintext localhost:50051 list`
- Dashboard issues: Check browser console + Socket.io status

## 9. Upgrades

- Blue-green deployments via Kubernetes
- Database migrations with EF Core
- Rust: `cargo update`

## Compliance & Certifications

- SOC 2, ISO 27001 ready mappings
- Audit logs for PCI-DSS

For advanced topics (e.g., AI integration, custom scanners), refer to `docs/ARCHITECTURE.md` and `docs/EXTENDING.md`.

---

**Last Updated**: June 2025
**Version**: 1.0 Production
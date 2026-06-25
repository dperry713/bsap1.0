# Threat Model for BSAP

## Assets
- Scan results, sensitive findings
- Secrets in scanned targets

## Threats
- Unauthorized scan access
- Data exfiltration
- Scanner compromise

## Mitigations
- RBAC, mTLS, encryption
- Sandboxed scanners
- Audit logging

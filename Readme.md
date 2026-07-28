# CI/CD Playground 🚀

A demonstration project showcasing a modern CI/CD pipeline for a Rust application using **GitHub Actions**, **Docker**, and **DevSecOps** practices.

The primary goal of this repository is not the application itself, but the automated pipeline that validates, secures, packages, tests, and scans every build before it is ready for deployment.

---

## Pipeline Overview

```text
                ┌─────────────┐
                │ Source Code │
                └──────┬──────┘
                       │
                       ▼
            Checkout Repository
                       │
                       ▼
             Install Rust Toolchain
                       │
                       ▼
      ┌────────────────────────────────┐
      │ Code Quality Validation        │
      │ • cargo fmt                    │
      │ • cargo clippy                 │
      └────────────────────────────────┘
                       │
                       ▼
             Build Docker Image
                       │
                       ▼
      ┌────────────────────────────────┐
      │ Security Scanning              │
      │ • Trivy SARIF Report           │
      │ • GitHub Security Dashboard    │
      │ • Fail on High/Critical CVEs   │
      └────────────────────────────────┘
                       │
                       ▼
             Start Container
                       │
                       ▼
      ┌────────────────────────────────┐
      │ Runtime Validation             │
      │ • Health Check                 │
      │ • OWASP ZAP Baseline Scan      │
      └────────────────────────────────┘
                       │
                       ▼
            Upload Security Reports
                       │
                       ▼
          Push Image to GHCR
```

---

# CI/CD Workflow

The workflow performs a complete build validation before publishing a Docker image.

| Stage | Tool | Purpose |
|--------|------|---------|
| Checkout | GitHub Actions | Clone repository |
| Setup | Rust Toolchain | Install latest stable Rust |
| Formatting | cargo fmt | Enforce code formatting |
| Static Analysis | Clippy | Detect Rust code issues |
| Build | Docker | Build container image |
| Vulnerability Scan | Trivy | Scan OS & Rust dependencies |
| Security Upload | SARIF | Publish findings to GitHub Security |
| Runtime Scan | OWASP ZAP | Dynamic web application scan |
| Artifact Upload | GitHub Actions | Upload ZAP HTML report |
| Publish | GHCR | Push validated Docker image |

---

# Security Pipeline

This repository follows a simple DevSecOps workflow.

## Static Security

✅ Rust formatting checks

✅ Clippy lint analysis

---

## Container Security

**Trivy** performs two scans.

### Report Generation

Produces a SARIF report that is uploaded to GitHub Security.

- Does **not** fail the pipeline
- Provides visibility into vulnerabilities

### Build Gate

Runs another scan that fails the workflow if any

- HIGH
- CRITICAL

vulnerabilities are found.

---

## Dynamic Application Security Testing (DAST)

After the container starts successfully:

- waits for the application
- verifies it is reachable
- launches an **OWASP ZAP Baseline Scan**
- uploads an HTML report as a workflow artifact

This validates the running application rather than only scanning the image.

---

# GitHub Actions Workflow

```
Developer
    │
    ▼
GitHub Actions
    │
    ├── cargo fmt
    ├── cargo clippy
    ├── docker build
    ├── Trivy Scan
    ├── Upload SARIF
    ├── Run Container
    ├── Health Check
    ├── OWASP ZAP
    ├── Upload Report
    └── Push to GHCR
```

---

# Technologies

- Rust
- Docker
- GitHub Actions
- GitHub Container Registry (GHCR)
- Trivy
- OWASP ZAP
- SARIF
- GitHub Code Scanning

---

# Repository Structure

```
.
├── .github
│   └── workflows
│       └── docker.yml
├── src
├── Cargo.toml
├── Dockerfile
└── README.md
```

---

# Running Locally

```bash
cargo run
```

Build Docker image

```bash
docker build -t ci-cd-playground .
```

Run container

```bash
docker run -p 8080:8080 ci-cd-playground
```

---

# GitHub Security Features

The workflow integrates with GitHub Security by uploading:

- Trivy SARIF results
- Code Scanning Alerts
- OWASP ZAP HTML report

This provides centralized visibility into security findings directly within the repository.

---

# Future Improvements

- Unit test stage
- Integration testing
- Docker image signing (Cosign)
- SBOM generation
- Multi-stage release pipeline
- Semantic versioning
- Automatic release creation
- Kubernetes deployment
- Helm deployment
- Terraform infrastructure provisioning

---

# Purpose

This project is intended as a practical demonstration of an end-to-end CI/CD pipeline that incorporates:

- Code quality
- Containerization
- Security scanning
- Runtime validation
- Artifact publishing
- Container registry integration

It serves as a reference implementation for DevOps and Platform Engineering workflows.
# Modulo

Modular multi-tenant platform for daily sales reconciliation.

## Layout
- `crates/domain`: reconciliation logic
- `crates/ingestion`: Restomax export and Z-report parsing
- `crates/api`: Axum API
- `web/`: Svelte frontend
- `docs/adr/`: architecture decision records

## Run locally
    cargo build
    cd web && npm ci && npm run dev

## Workflow
Short-lived branches from `main`, one pull request per Linear issue, squash merge.
Branch names come from Linear. Commits follow Conventional Commits and mention the issue, for example `feat(api): add health route (MD-18)`.

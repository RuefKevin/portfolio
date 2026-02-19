# Portfolio

Personal portfolio website with focus on security best practices.

## Tech Stack

- **Backend:** Rust + Axum
- **Frontend:** SvelteKit + TypeScript
- **Database:** SQLite

## Security Features

- Signed commits (GPG)
- Automated dependency auditing (cargo-audit, npm audit)
- Secret scanning (gitleaks)
- Security headers from day one
- Rate limiting
- Input validation

## Local Development

## Prerequisites

- Rust 1.75+
- Node.js 20+
- SQLite3

### Setup

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
npm install
npm run dev

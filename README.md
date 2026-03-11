# Portfolio

Personal portfolio website built with a security-first mindset.

## Tech Stack

- **Backend:** Rust + Axum + SQLx + SQLite
- **Frontend:** SvelteKit + TypeScript
- **Infrastructure:** Docker + GitHub Actions

## Security Features

- Content Security Policy (CSP)
- Security Headers (X-Frame-Options, X-Content-Type-Options, Referrer-Policy, HSTS)
- CORS restricted to allowed origins
- Rate limiting (via tower_governor)
- Automated dependency auditing (cargo-audit, npm audit)
- Secret scanning (Gitleaks)
- Non-root Docker containers
- Signed commits (GPG)
- Dependabot for Cargo, npm and GitHub Actions

## Project Structure
```
portfolio/
├── backend/        # Rust/Axum REST API
│   ├── migrations/ # SQLite migrations and seed data
│   ├── src/        # Application code
│   └── tests/      # Integration tests
├── frontend/       # SvelteKit application
│   └── src/
│       └── routes/ # Page routes
└── docker-compose.yml
```

## Local Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- SQLite3
- C linker and build tools (required for Rust compilation)

### Setup

1. Clone the repository
```bash
git clone https://github.com/RuefKevin/portfolio.git
cd portfolio
```

2. Configure environment
```bash
cp .env.example .env
cp frontend/.env.example frontend/.env
```

Edit `.env`:
```bash
HOST=127.0.0.1
PORT=3000
DATABASE_URL=sqlite:./portfolio.db
ALLOWED_ORIGINS=http://localhost:5173
RATE_LIMIT=false
```

Edit `frontend/.env`:
```bash
BACKEND_URL=http://localhost:3000
CONTACT_EMAIL=your@email.com
CONTACT_GITHUB=github.com/username
CONTACT_LINKEDIN=linkedin.com/in/username
```

3. Create the database and run migrations
```bash
cd backend
sqlx database create --database-url sqlite:./portfolio.db
cargo run
```

On first startup, migrations run automatically and seed data is inserted.

4. Start frontend
```bash
cd frontend && npm install && npm run dev
```

### Adding your own data

Projects and blog posts are stored in SQLite and managed via migrations in `backend/migrations/`.
Edit the seed `INSERT` statements in the migration files before the first run, 
or insert directly into the database afterwards:
```bash
sqlite3 backend/portfolio.db
```

### SQLx offline mode

This project uses SQLx compile-time query checking. If you add new queries, regenerate the cache:
```bash
cd backend && cargo sqlx prepare --database-url sqlite:./portfolio.db
```

The generated `.sqlx/` files must be committed so CI can build without a live database.

## Docker
```bash
docker compose up --build
```

The app will be available at `http://localhost:3000`. 
The database is created automatically on first startup via a Docker volume.

## CI/CD

Four GitHub Actions jobs run on every push to `develop` and `main`:

- **backend-build** – cargo check, clippy, tests
- **backend-audit** – cargo-audit security scan
- **frontend-build** – type check, lint, tests
- **frontend-audit** – npm audit security scan

## Environment Variables

### Backend (`.env`)

| Variable | Description | Default |
|---|---|---|
| `HOST` | Server host | `127.0.0.1` |
| `PORT` | Server port | `3000` |
| `DATABASE_URL` | SQLite database path | – |
| `ALLOWED_ORIGINS` | Comma-separated allowed CORS origins | – |
| `RATE_LIMIT` | Enable rate limiting | `true` |

### Frontend (`frontend/.env`)

| Variable | Description |
|---|---|
| `BACKEND_URL` | Backend API URL |
| `CONTACT_EMAIL` | Contact email address |
| `CONTACT_GITHUB` | GitHub profile URL |
| `CONTACT_LINKEDIN` | LinkedIn profile URL |
# 📝 todo_list

**todo_list** is a task management service written in [Rust](https://www.rust-lang.org/) using [Axum](https://github.com/tokio-rs/axum) and [SQLx](https://github.com/launchbadge/sqlx).  
It uses **PostgreSQL** as the database and provides API documentation via **OpenAPI/Swagger**.

The project is structured as a **Rust workspace** and follows the **Hexagonal Architecture (Ports & Adapters)** principle:
- `domain` — the application core (use cases, business logic, entities)
- `input_adapter` — input adapters (HTTP API with Axum)
- `output_adapter` — output adapters (SQLx for PostgreSQL)

---

## 📦 Features

- CRUD for tasks (`create`, `list`, `get`, `update`, `delete`)
- PostgreSQL + SQLx (async/await, offline mode)
- Hexagonal architecture (domain independent from infrastructure)
- Workspace structure for modularity
- Swagger UI (`/docs`) with OpenAPI specification
- Docker + Docker Compose support

---

## 🗂 Project structure

```text
todo_list/
├── .sqlx/             # query metadata for offline usage
├── domain/            # core domain (use cases, entities)
├── input_adapter/     # HTTP API (Axum handlers)
├── output_adapter/    # SQLx queries/commands
├── migrations/        # SQLx migrations
├── swagger/           # Swagger UI static files + openapi.yaml
├── entrypoint.sh      # startup script for Docker runtime
├── Cargo.toml         # workspace manifest
├── Dockerfile
├── docker-compose.yml
└── README.md
```

---

## 🚀 Running locally

### Requirements
- Rust 1.89.0
- PostgreSQL 17
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

### Steps

```text
# 1. Clone the repo

git clone https://github.com/M0o4/todo_list_hexagon.git
cd todo_list_hexagon

# 2. Start PostgreSQL (example with Docker)
docker run --rm -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=app_db -p 5432:5432 postgres:17

# 3. Create .env file
echo "DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db" > .env
echo "RUST_LOG=debug" >> .env

# 4. Run migrations
sqlx migrate run

# 5. Start the service
cargo run

# Open in browser:
Swagger UI: http://localhost:3000/docs/
```

---

## 🐳 Running with Docker Compose

### Requirements
- Docker 
- Docker Compose

### Steps
```text
# Build and start containers
docker compose up --build

# View logs
docker compose logs -f app

# Access:
Swagger UI: http://localhost:3000/docs/
```

---

## ✅ Useful commands
```text
# Prepare SQLx offline cache
cargo sqlx prepare --workspace

# Run migrations
sqlx migrate run

# Rebuild Docker without cache
docker compose build --no-cache app
```
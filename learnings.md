# 🦀 Rust CRUD API - Learning Journey

This document tracks key learnings from building a REST API with Rust, Axum, and PostgreSQL.

---

## 📋 Learning Progress

| Step | Topic | Status | Commit |
|------|-------|--------|--------|
| 1 | Axum Server Setup | ✅ Complete | Ready |
| 2 | Health Check Route | ✅ Complete | Ready |
| 3 | PostgreSQL Connection | ✅ Complete | Ready |
| 4 | Database Schema | ⏳ Not Started | - |
| 5 | User Struct | ⏳ Not Started | - |
| 6 | Create User (POST) | ⏳ Not Started | - |
| 7 | Get User (GET) | ⏳ Not Started | - |
| 8 | Update User (PUT) | ⏳ Not Started | - |
| 9 | Delete User (DELETE) | ⏳ Not Started | - |
| 10 | Error Handling | ⏳ Not Started | - |
| 11 | API Container | ⏳ Not Started | - |
| 12 | Database Container | ⏳ Not Started | - |
| 13 | Docker Compose | ⏳ Not Started | - |

---

## 📚 Step-by-Step Learnings

### Step 1: Axum Server Setup ✅

**Goal:** Create a basic Axum server that starts and listens on a port.

**What I Learned:**

#### 1. **Async Rust with Tokio**
```rust
#[tokio::main]
async fn main() { ... }
```
- `#[tokio::main]` is a **macro** that sets up the async runtime
- It transforms your async `main()` into a synchronous entry point
- Without this, Rust doesn't know how to run async code
- Think of it as: "Hey Tokio, please manage all my async operations"

#### 2. **Axum's Router**
```rust
let app = Router::new();
```
- `Router` is the core of Axum - it maps URLs to handlers
- Right now it's empty (no routes), but it's the foundation
- We'll add routes like `/health`, `/users` in later steps
- Router is what makes Axum so clean and composable

#### 3. **TCP Listener**
```rust
let listener = TcpListener::bind("127.0.0.1:3000").await
```
- `TcpListener` binds to an IP address and port
- `127.0.0.1` = localhost (your computer only)
- Port `3000` is where the server listens for requests
- `.await` means "pause here until binding completes"
- This is **async** - it doesn't block other operations

#### 4. **Serving the App**
```rust
axum::serve(listener, app).await
```
- Takes the listener and router, starts accepting connections
- This line **blocks** indefinitely (server runs forever)
- Each incoming request gets routed through `app`
- Right now no routes exist, so nothing happens yet!

#### 5. **The `.await` Keyword**
- Used on async operations that might take time
- Tells Rust: "Pause this task, let other tasks run, resume when ready"
- Network operations (binding, serving) are async because they wait for external events
- This is how Rust handles many connections efficiently

**Key Dependencies Added:**
- `axum = "0.7"` - Web framework built on top of Tokio
- `tokio = { version = "1", features = ["full"] }` - Async runtime that makes async/await work

**What Works Now:**
- ✅ Server compiles and starts
- ✅ Listens on `http://127.0.0.1:3000`
- ✅ Prints startup message
- ⚠️ No routes yet - visiting the URL returns nothing

**Next Up:** Add a `/health` route so we can actually test the server!

---

### Step 2: Health Check Route ✅

**Goal:** Add a `/health` endpoint and learn professional code organization.

**What I Learned:**

#### 1. **Routing in Axum**
```rust
Router::new()
    .route("/health", get(health))
```
- `.route(path, method_handler)` maps URLs to handler functions
- `get()` specifies this route only responds to GET requests
- Other methods: `post()`, `put()`, `delete()`, `patch()`
- Routes are defined declaratively and chained together

#### 2. **Handler Functions**
```rust
async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}
```
- Handlers are async functions that return responses
- Return type can be a tuple: `(StatusCode, Json<T>)`
- `StatusCode::OK` = HTTP 200
- `Json<T>` automatically serializes data to JSON
- Must be async because they might do I/O operations

#### 3. **JSON Responses with serde_json**
```rust
Json(json!({
    "status": "ok",
    "message": "Server is healthy"
}))
```
- `json!` macro creates JSON objects easily
- `Json` wrapper tells Axum to set `Content-Type: application/json`
- Automatic serialization - no manual string building needed!

#### 4. **Professional Code Structure** 🏗️
Refactored into modules for scalability:

```
src/
├── main.rs           # Entry point (server startup only)
├── lib.rs            # Library exports
├── routes.rs         # All route definitions
└── handlers/
    ├── mod.rs        # Handler module index
    └── health.rs     # Health check handler
```

**Why this structure?**
- **Separation of Concerns**: Each file has one responsibility
- **Scalability**: Easy to add new handlers and routes
- **Testability**: Can test handlers independently
- **Maintainability**: Find code quickly, understand organization
- **Professional**: Industry-standard Rust project layout

#### 5. **Module System**
```rust
// lib.rs
pub mod handlers;
pub mod routes;

// main.rs
use rust_crud::routes;
```
- `lib.rs` exposes modules as a library
- `mod.rs` in a folder makes it a module
- `pub mod` makes modules publicly accessible
- `use` imports from your own library

#### 6. **File Additions for Production**
- **`.gitignore`**: Don't commit build artifacts, secrets, IDE files
- **`.env.example`**: Template for environment configuration
- Shows what config is needed without exposing secrets

**Key Dependencies:**
- `serde_json = "1.0"` - JSON serialization/deserialization

**What Works Now:**
- ✅ `/health` endpoint returns JSON
- ✅ Code is modular and organized
- ✅ Easy to add new endpoints
- ✅ Professional project structure
- ✅ Ready for team collaboration

**Testing:**
```bash
curl http://127.0.0.1:3000/health
# Returns: {"status":"ok","message":"Server is healthy"}
```

**Next Up:** Connect to PostgreSQL database!

---

### Step 3: PostgreSQL Connection ✅

**Goal:** Connect Rust API to PostgreSQL database using Docker and SQLx.

**What I Learned:**

#### 1. **Docker Compose for Databases**
```yaml
services:
  postgres:
    image: postgres:16-alpine
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
```
- **Why Docker?** Consistent database environment, easy to start/stop
- **Volumes**: Persist data even when container stops
- **Health Checks**: Verify database is ready before connecting
- **Alpine**: Lightweight Linux distribution, smaller image size

#### 2. **Environment Variables with dotenvy**
```rust
dotenvy::dotenv().ok();
let url = std::env::var("DATABASE_URL")?;
```
- **`.env` file**: Store secrets locally (gitignored!)
- **`dotenvy`**: Loads `.env` into environment variables
- **`.ok()`**: Ignore errors if `.env` doesn't exist (for production)
- **Security**: Never commit database credentials to git!

#### 3. **SQLx - Async SQL Toolkit**
```rust
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "macros"] }
```
- **`runtime-tokio`**: Use Tokio for async operations
- **`postgres`**: PostgreSQL driver
- **`macros`**: Compile-time SQL validation (catches errors early!)
- Fully async - no blocking the event loop

#### 4. **Connection Pooling**
```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(30))
    .connect(&database_url)
    .await?;
```
- **Why Pool?** Creating connections is slow/expensive
- **How it works**: Maintains 5 ready-to-use connections
- **Benefits**: Much faster than connecting per-request
- **Timeout**: Don't wait forever if database is down

#### 5. **Shared State in Axum**
```rust
// In main.rs
let app = routes::create_routes().with_state(pool);

// In handler
pub async fn health(State(pool): State<PgPool>) -> Response {
    // pool is available here!
}
```
- **`.with_state()`**: Makes data available to all handlers
- **`State<T>` extractor**: Pulls state from request
- **Type safety**: Compiler ensures types match
- **Perfect for**: Database pools, config, shared resources

#### 6. **Database Queries**
```rust
sqlx::query("SELECT 1")
    .fetch_one(&pool)
    .await
```
- **`sqlx::query()`**: Execute raw SQL
- **`.fetch_one()`**: Get exactly one row
- **`.await`**: Async operation, doesn't block
- Returns `Result` for error handling

#### 7. **Module Organization**
```
src/
├── db.rs          # Database connection logic (NEW!)
├── handlers/      # Handler functions
├── routes.rs      # Route definitions  
├── lib.rs         # Module exports
└── main.rs        # Entry point
```
- **`db.rs`**: Centralized database setup
- Easy to add more DB functions later
- Separation of concerns

**Key Dependencies Added:**
- `sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "macros"] }`
- `dotenvy = "0.15"` 

**New Files Created:**
- `docker-compose.yml` - PostgreSQL container definition
- `.env` - Local environment variables (gitignored)
- `src/db.rs` - Database connection pool setup
- `SETUP_STEP3.md` - Manual setup instructions

**What Works Now:**
- ✅ PostgreSQL runs in Docker container
- ✅ Application connects to database on startup
- ✅ Connection pool ready for queries
- ✅ `/health` verifies database connection
- ✅ Proper error handling

**Testing:**
```bash
# Start database
docker compose up -d

# Run app
cargo run

# Test health check
curl http://127.0.0.1:3000/health
# Returns: {"status":"ok","database":"connected"}
```

**Key Concepts Learned:**
- **Connection Pooling**: Reuse connections for performance
- **Async Database Operations**: Non-blocking queries with SQLx
- **State Management**: Sharing data across handlers in Axum
- **Environment Configuration**: Secure credential management
- **Container Orchestration**: Running services with Docker Compose

**Next Up:** Create the `users` table with SQL migrations!

---

*More learnings will be added as we progress through each step...*

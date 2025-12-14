# 🦀 Rust CRUD API - Learning Journey

This document tracks key learnings from building a REST API with Rust, Axum, and PostgreSQL.

---

## 📋 Learning Progress

| Step | Topic | Status | Commit |
|------|-------|--------|--------|
| 1 | Axum Server Setup | ✅ Complete | Ready |
| 2 | Health Check Route | ✅ Complete | Ready |
| 3 | PostgreSQL Connection | ⏳ Not Started | - |
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

*More learnings will be added as we progress through each step...*

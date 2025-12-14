# 🦀 Rust CRUD API - Learning Journey

This document tracks key learnings from building a REST API with Rust, Axum, and PostgreSQL.

---

## 📋 Learning Progress

| Step | Topic | Status | Commit |
|------|-------|--------|--------|
| 1 | Axum Server Setup | ✅ Complete | Ready |
| 2 | Health Check Route | ⏳ Not Started | - |
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

*More learnings will be added as we progress through each step...*

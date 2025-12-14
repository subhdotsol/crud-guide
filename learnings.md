# 🦀 Rust CRUD API - Learning Journey

This document tracks key learnings from building a REST API with Rust, Axum, and PostgreSQL.

---

## 📋 Learning Progress

| Step | Topic | Status | Commit |
|------|-------|--------|--------|
| 1 | Axum Server Setup | ✅ Complete | Ready |
| 2 | Health Check Route | ✅ Complete | Ready |
| 3 | PostgreSQL Connection | ✅ Complete | Ready |
| 4 | Database Schema | ✅ Complete | Ready |
| 5 | User Struct | ✅ Complete | Ready |
| 6 | Create User (POST) | ✅ Complete | Ready |
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

### Step 4: Create Users Table ✅

**Goal:** Learn database migrations and create a users table with proper schema.

**What I Learned:**

#### 1. **SQLx CLI Tool**
```bash
cargo install sqlx-cli --no-default-features --features postgres
```
- **What it does**: Command-line tool for managing migrations
- **`--no-default-features`**: Don't install all database drivers
- **`--features postgres`**: Only PostgreSQL support
- Installed binaries: `sqlx` and `cargo-sqlx`

#### 2. **Creating Migrations**
```bash
sqlx migrate add create_users_table
```
- Creates timestamped file: `migrations/TIMESTAMP_create_users_table.sql`
- **Timestamp**: Ensures migrations run in order
- **Descriptive name**: Makes it easy to understand what it does
- **Version control**: Track database changes in git!

#### 3. **SQL Schema Design**
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**Field by Field:**
- `id SERIAL PRIMARY KEY`
  - **SERIAL**: Auto-incrementing integer (1, 2, 3...)
  - **PRIMARY KEY**: Unique identifier + indexed + not null
- `name VARCHAR(255) NOT NULL`
  - **VARCHAR(255)**: Variable-length string up to 255 chars
  - **NOT NULL**: Required field (can't be empty)
- `email VARCHAR(255) NOT NULL UNIQUE`
  - **UNIQUE**: No duplicate emails allowed
  - Database enforces uniqueness
- `age INTEGER`
  - **INTEGER**: Whole number
  - **Nullable**: Optional field (no NOT NULL)
- `created_at TIMESTAMP WITH TIME ZONE`
  - **TIMESTAMP**: Date + time
  - **WITH TIME ZONE**: Stores timezone info
  - **DEFAULT CURRENT_TIMESTAMP**: Auto-set on insert

#### 4. **Indexes for Performance**
```sql
CREATE INDEX idx_users_email ON users(email);
```
- **Index**: Makes lookups fast (like a book index)
- Searching by email is now O(log n) instead of O(n)
- Unique constraint already creates index, but explicit is clearer

#### 5. **Running Migrations**
```bash
sqlx migrate run
```
- Connects to database using `DATABASE_URL`
- Checks which migrations already ran
- Runs new migrations in order
- Creates `_sqlx_migrations` table to track what's been run

**Migration tracking table:**
```
_sqlx_migrations
├── version (timestamp from filename)
├── description
├── installed_on
└── success
```

#### 6. **Verifying Schema**
```bash
docker exec -it rust-crud-postgres psql -U postgres -d rust_crud -c "\d users"
```
- **\d users**: Describe table command
- Shows columns, types, constraints
- Confirms table structure matches migration

#### 7. **Migrations as Version Control**
- Each migration = one database change
- Never edit old migrations (breaks history)
- To change: create new migration
- Can rollback with `sqlx migrate revert`
- Team members get same database structure

**Key Concepts:**
- **Idempotent**: Safe to run multiple times
- **Ordered**: Applied in timestamp order
- **Tracked**: System knows what's applied
- **Reversible**: Can rollback changes

**New Files Created:**
- `migrations/20251214202409_create_users_table.sql` - Migration file
- `.sqlx/` directory - SQLx metadata (gitignored in production)

**Database Tables:**
- `users` - Our application data
- `_sqlx_migrations` - Migration tracking

**What Works Now:**
- ✅ Users table exists in database
- ✅ Schema has proper constraints
- ✅ Email uniqueness enforced
- ✅ Auto-incrementing IDs
- ✅ Timestamps track creation time
- ✅ Ready to insert data!

**Verification:**
```sql
-- Table exists
SELECT tablename FROM pg_tables WHERE schemaname = 'public';
-- Returns: users, _sqlx_migrations

-- Table structure correct
\d users
-- Shows all columns with correct types
```

**Next Up:** Create Rust `User` struct to map to this table!

---

### Step 5: Create User Struct ✅

**Goal:** Define Rust structs that map to database tables with type safety.

**What I Learned:**

#### 1. **Model Organization**
```
src/
├── models/
│   ├── mod.rs          # Module exports
│   └── user.rs         # User model
```
- **models/**: Central location for all data structures
- **Separation**: Each entity gets its own file
- **Reusable**: Import models anywhere with `use rust_crud::models::User;`

#### 2. **The User Struct**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Field Mapping:**
| Rust Type | SQL Type | Why |
|-----------|----------|-----|
| `i32` | SERIAL/INTEGER | 32-bit signed integer |
| `String` | VARCHAR(255) | Owned string (heap-allocated) |
| `Option<i32>` | INTEGER (nullable) | Optional value (Some or None) |
| `DateTime<Utc>` | TIMESTAMP WITH TIME ZONE | Timezone-aware datetime |

#### 3. **Derive Macros** 🪄
```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
```
Each derive adds functionality:

- **`Debug`**: Print struct with `{:?}` for debugging
- **`Clone`**: Make copies of the struct
- **`Serialize`** (serde): Convert struct → JSON
- **`Deserialize`** (serde): Convert JSON → struct
- **`FromRow`** (sqlx): Map database row → struct

**Why derives are magical:**
- Compiler generates code for you
- No boilerplate to write
- Type-safe conversions

#### 4. **Serde for JSON**
```rust
use serde::{Deserialize, Serialize};
```
- **Serialize**: Rust struct → JSON (for responses)
- **Deserialize**: JSON → Rust struct (from requests)
- **Automatic**: Works with Axum's `Json` extractor

**Example:**
```rust
// Automatically becomes JSON
let user = User { ... };
Json(user) // Returns: {"id": 1, "name": "Alice", ...}
```

#### 5. **Chrono for Dates**
```rust
use chrono::{DateTime, Utc};
pub created_at: DateTime<Utc>
```
- **DateTime<Utc>**: Timezone-aware datetime
- **Utc**: Coordinated Universal Time (UTC+0)
- **Serialize**: Becomes ISO 8601 string in JSON
  - Example: `"2025-12-14T20:24:09Z"`

#### 6. **DTOs (Data Transfer Objects)**
```rust
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
}
```

**Why separate DTOs?**
- **CreateUser**: No `id` or timestamps (database generates these)
- **UpdateUser**: All fields optional (update only what changed)
- **User**: Full model with all fields

**Prevents errors:**
- Client can't set their own ID ✅
- Client can't fake timestamps ✅
- Type system enforces correctness ✅

#### 7. **UpdateUser DTO**
```rust
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}
```
- **All fields `Option<T>`**: Update only specified fields
- **Flexible**: Can update name only, or email + age, etc.
- **Type-safe**: Can't update invalid fields

#### 8. **SQLx FromRow**
```rust
#[derive(FromRow)]
```
Automatically maps SQL columns to Rust fields:
```sql
SELECT id, name, email, age, created_at, updated_at FROM users;
```
→ Becomes `User` struct ✨

**Rules:**
- Field names must match column names
- Types must be compatible
- Compile-time checking with `query_as!` macro

#### 9. **Module Exports**
```rust
// src/models/mod.rs
pub mod user;
pub use user::{CreateUser, UpdateUser, User};
```

**Now you can:**
```rust
use rust_crud::models::User;          // ✅
use rust_crud::models::{User, CreateUser}; // ✅
```

Instead of:
```rust
use rust_crud::models::user::User;    // ❌ Longer
```

**Key Dependencies Added:**
- `serde = { version = "1.0", features = ["derive"] }` - JSON serialization
- `chrono = { version = "0.4", features = ["serde"] }` - Datetime handling

**New Files Created:**
- `src/models/mod.rs` - Module exports
- `src/models/user.rs` - User model and DTOs

**What Works Now:**
- ✅ User struct represents database row
- ✅ Automatic JSON serialization
- ✅ Type-safe database mapping
- ✅ DTOs for create/update operations
- ✅ Timestamp handling with timezone
- ✅ Optional fields with `Option<T>`

**Type Safety Benefits:**
```rust
// Won't compile! ✋
let user = User {
    id: "not a number",  // ❌ Type error: expected i32
    name: 123,           // ❌ Type error: expected String
    age: 25,             // ❌ Type error: expected Option<i32>
};

// Correct! ✅
let user = User {
    id: 1,
    name: "Alice".to_string(),
    age: Some(25),       // ✅ Optional value
    // ... rest
};
```

**Next Up:** Implement POST endpoint to create users!

---

### Step 6: Create User Endpoint (POST) ✅

**Goal:** Implement POST /users endpoint to create new users in the database.

**What I Learned:**

#### 1. **HTTP POST Handler**
```rust
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)>
```

**Breaking it down:**
- `State<PgPool>`: Get database pool from Axum state
- `Json<CreateUser>`: Automatically deserialize JSON request body to CreateUser
- Returns `Result<Success, Error>`
- **Success**: `(StatusCode::CREATED, Json<User>)` - 201 status + created user
- **Error**: `(StatusCode, String)` - Error status + message

#### 2. **SQLx query_as**
```rust
let user = sqlx::query_as::<_, User>(
    r#"
    INSERT INTO users (name, email, age)
    VALUES ($1, $2, $3)
    RETURNING id, name, email, age, created_at, updated_at
    "#,
)
.bind(&payload.name)
.bind(&payload.email)
.bind(payload.age)
.fetch_one(&pool)
.await?;
```

**Key parts:**
- **`query_as::<_, User>`**: Type annotation tells SQLx to map result to User struct
- **`r#"..."`#**: Raw string literal (no need to escape quotes)
- **`$1, $2, $3`**: Positional parameters (prevents SQL injection!)
- **`.bind()`**: Bind values to parameters in order
- **`RETURNING *`**: Get the inserted row back (with generated id!)
- **`.fetch_one()`**: Expect exactly one row back
- **`.await?`**: Wait for query + propagate errors with `?`

#### 3. **RETURNING Clause**
```sql
INSERT INTO users (name, email, age)
VALUES ($1, $2, $3)
RETURNING id, name, email, age, created_at, updated_at
```

**Why RETURNING is awesome:**
- Database generates `id` (SERIAL auto-increment)
- Database sets timestamps with `DEFAULT`
- Get all values back in one roundtrip
- No need for separate SELECT query!

#### 4. **Error Handling**
```rust
.map_err(|e| {
    eprintln!("Database error: {:?}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to create user: {}", e),
    )
})?;
```

- **`.map_err()`**: Transform database error to HTTP error
- **`eprintln!()`**: Log error to console for debugging
- **500 error**: Server error (not client's fault)
- **Error message**: Helpful for debugging

#### 5. **HTTP Status Codes**
```rust
StatusCode::CREATED  // 201 - Resource created successfully
StatusCode::INTERNAL_SERVER_ERROR  // 500 - Server error
```

- **201 Created**: Successful creation (better than 200 OK)
- **Semantic**: Different codes have different meanings
- **Axum provides**: All standard HTTP status codes

#### 6. **JSON Request/Response**
**Request (automatic):**
```json
{
  "name": "Alice",
  "email": "alice@example.com",
  "age": 25
}
```
→ Axum deserializes to `CreateUser` ✨

**Response (automatic):**
```rust
Json(user)  // User struct
```
→ Axum serializes to JSON ✨
```json
{
  "id": 1,
  "name": "Alice", 
  "email": "alice@example.com",
  "age": 25,
  "created_at": "2025-12-14T20:55:40.298920Z",
  "updated_at": "2025-12-14T20:55:40.298920Z"
}
```

#### 7. **SQL Injection Prevention**
```rust
// ❌ NEVER DO THIS! (SQL injection vulnerability)
let query = format!("INSERT INTO users VALUES ('{}')", payload.name);

// ✅ ALWAYS DO THIS! (safe with parameterized query)
.bind(&payload.name)  // SQLx handles escaping
```

- **Parameterized queries**: $1, $2, etc.
- **SQLx escapes**: Prevents injection attacks
- **Type-safe**: Compile-time checking with macros

#### 8. **Adding chrono Feature**
```toml
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "macros", "chrono"] }
```

- **"chrono" feature**: Enables DateTime<Utc> support
- Without this: Can't map TIMESTAMP to DateTime
- SQLx integrates with chrono automatically

**New Files Created:**
- `src/handlers/users.rs` - Users CRUD handlers

**Modified Files:**
- `src/handlers/mod.rs` - Export users module
- `src/routes.rs` - Add POST /users route
- `Cargo.toml` - Add chrono feature to sqlx

**What Works Now:**
- ✅ POST /users creates users
- ✅ Database generates id and timestamps
- ✅ JSON request/response automatic
- ✅ Error handling with proper status codes
- ✅ SQL injection protection
- ✅ Type-safe database queries

**Testing:**
```bash
# Create user with age
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com", "age": 25}'

# Response: 
# {"id":1,"name":"Alice","email":"alice@example.com","age":25,...}

# Create user without age (optional field)
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob", "email": "bob@example.com"}'

# Response: 
# {"id":2,"name":"Bob","email":"bob@example.com","age":null,...}
```

**Database Verification:**
```sql
SELECT * FROM users;
-- id | name  | email             | age |  created_at  | updated_at
-- 1  | Alice | alice@example.com | 25  | 2025-12-14...| 2025-12-14...
-- 2  | Bob   | bob@example.com   | null| 2025-12-14...| 2025-12-14...
```

**Next Up:** Implement GET /users/:id to fetch a user by ID!

---

*More learnings will be added as we progress through each step...*

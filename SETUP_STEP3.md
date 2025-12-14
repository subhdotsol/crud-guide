# Step 3 Setup Instructions

You need to complete a few manual steps before the app will work:

## 1. Start Docker Desktop

Your Docker daemon isn't running. Please:
- Open **Docker Desktop** application on your Mac
- Wait for it to fully start (whale icon in menu bar should be steady, not animated)

## 2. Create .env File

The `.env` file is gitignored for security. You need to create it manually:

```bash
# Create .env file with database credentials
cat > .env << 'EOF'
# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_crud

# Server Configuration  
HOST=127.0.0.1
PORT=3000

# Logging
RUST_LOG=info
EOF
```

Or copy from the template:
```bash
cp .env.example .env
```

## 3. Start PostgreSQL Container

```bash
docker compose up -d
```

Expected output:
```
[+] Running 2/2
 ✔ Network rust-crud_default    Created
 ✔ Container rust-crud-postgres Started
```

Verify it's running:
```bash
docker ps
```

You should see `rust-crud-postgres` in the list.

## 4. Build and Run the App

```bash
cargo build
cargo run
```

Expected output:
```
✅ Database connection pool created
🚀 Server listening on http://127.0.0.1:3000
📍 Health check: http://127.0.0.1:3000/health
```

## 5. Test the Health Endpoint

```bash
curl http://127.0.0.1:3000/health
```

Should return:
```json
{"status":"ok","database":"connected"}
```

✅ If you see `"database":"connected"` - SUCCESS! Step 3 is complete!

---

## Troubleshooting

### If database shows "disconnected"
1. Check Docker container: `docker ps`
2. Check logs: `docker logs rust-crud-postgres`
3. Verify DATABASE_URL in .env is correct

### If "connection refused"
- Make sure PostgreSQL container is running
- Check port 5432 isn't used by another process

### If build fails
- Run `cargo clean`
- Try `cargo build` again

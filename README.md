# Rust CRUD API with Axum & PostgreSQL

This project is a **learning-first backend API** built to understand how CRUD works in Rust by actually building it step by step. The focus is on clarity, real-world patterns, and hands-on experience rather than abstractions or heavy ORMs.

---

## 🎯 Goal

Build a simple REST API that manages a `User` entity using **Axum** and **PostgreSQL**, while learning:

- Axum fundamentals
- Async Rust with Tokio
- Database interaction using SQL
- Real CRUD request/response flow
- Containerization with Docker

---

## 🧠 What You’ll Build

A REST API that supports:

- Creating a user
- Fetching a user by ID
- Updating a user
- Deleting a user

Each user has:
- `name`
- `age`
- `email`

All data is stored in PostgreSQL.

---

## 🛠️ Tech Stack

- **Rust**
- **Axum** — HTTP web framework
- **Tokio** — async runtime
- **SQLx** — async SQL toolkit
- **PostgreSQL** — database
- **Docker & Docker Compose** — containerization

---

## 🧱 Project Philosophy

- Build things **incrementally**
- Prefer **explicit SQL** over magic
- Keep code readable and beginner-friendly
- Learn by doing, not by abstracting too early

---

## 🪜 Step-by-Step Build Plan

### 1. Axum Server
- Set up a basic Axum server
- Verify the server starts and listens on a port

### 2. Health Check Route
- Add a `/health` endpoint
- Return a simple response to confirm the server is running

### 3. Connect to PostgreSQL
- Run PostgreSQL using Docker
- Configure database connection using environment variables
- Initialize a connection pool

### 4. Create `users` Table
- Add SQL migrations
- Create a `users` table with:
  - `id`
  - `name`
  - `age`
  - `email`
  - timestamps if needed

### 5. Create User Struct
- Define a Rust `User` struct
- Map database fields to Rust types
- Use it for request and response handling

---

## 🔁 CRUD Implementation

### Create User
- Endpoint to create a new user
- Validate input
- Insert data into the database
- Return the created user

### Get User by ID
- Endpoint to fetch a user by ID
- Handle the case where the user does not exist

### Update User
- Endpoint to update an existing user
- Allow updating one or more fields
- Return the updated user

### Delete User
- Endpoint to delete a user by ID
- Return an appropriate success or error response

---

## ⚠️ Error Handling

- Handle database errors gracefully
- Return proper HTTP status codes:
  - `400` for bad requests
  - `404` for not found
  - `500` for server errors
- Avoid panics in request handlers

---

## 🐳 Containerization

### API Container
- Create a Dockerfile for the Rust API
- Build and run the server inside a container

### Database Container
- Run PostgreSQL using Docker Compose
- Persist data using volumes

### Docker Compose
- Run API and database together
- Simplify local development and setup

---

## 📚 What You Learn By Completing This

By building this project step by step, you will learn:

- How Axum handles routing and state
- How async Rust works in a real backend
- How to interact with a SQL database from Rust
- How CRUD APIs are designed and implemented
- How to containerize a backend service and database

---

## 🚀 Final Outcome

At the end of this project, you will have:
- A working CRUD API written in Rust
- A solid understanding of Axum and async Rust
- Hands-on experience with PostgreSQL and SQL
- A containerized backend ready for further expansion

This project is intentionally simple so it can be extended later with authentication, pagination, validation, or caching.

Happy building 🦀
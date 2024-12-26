# Rust CRUD API Example

A simple CRUD API built using Rust. This project is designed for learning and practicing Rust programming, focusing on:
- Modular code organization
- Handling HTTP requests
- Database interaction with PostgreSQL
- Using Serde for JSON serialization/deserialization

## Features
- **Create** a new user (POST `/users`)
- **Retrieve** a single user by ID (GET `/users/:id`)
- **Retrieve all** users (GET `/users`)
- **Update** an existing user by ID (PUT `/users/:id`)
- **Delete** a user by ID (DELETE `/users/:id`)

## Tech Stack
- Language: **Rust**
- Database: **PostgreSQL**
- Dependencies:
  - `postgres` for database connectivity
  - `serde` and `serde_json` for JSON handling
  - `serde_derive` for deriving serialization traits

## Install dependencies 
    `cargo build`

## Run
    `cargo run`
# Loco Web Learning Project

This project is a personal learning environment for exploring modern web development in Rust using **Loco**, an opinionated and batteries‑included framework. The goal is to build a clean, structured, and scalable backend while understanding how Loco organizes applications and accelerates development.

## Learning Goals

- Understand the fundamentals of **Loco** and its opinionated application structure
- Learn how Loco organizes:
  - Controllers
  - Models
  - Services
  - Jobs & queues
  - Configuration
  - Error handling
- Work with **database models and migrations** using Diesel or SeaORM (depending on project setup)
- Build RESTful APIs with consistent patterns for requests and responses
- Implement **authentication** (JWT, sessions, role‑based access)
- Explore built‑in features such as:
  - Background jobs
  - Email sending
  - CLI scaffolding
  - Tracing and logging
- Practice clean architecture and modular design within Loco’s conventions
- Learn to use environment configuration, Docker, and development tooling provided by Loco

## What This Project Covers

- Loco’s directory structure and how each module fits into the application
- Using the **Loco CLI** to scaffold:
  - Controllers
  - Models
  - Migrations
  - Jobs
- Working with the built‑in **App** lifecycle and dependency injection
- Implementing validation and error handling using Loco’s utilities
- Creating background jobs and scheduling asynchronous tasks
- Integrating database queries through Loco’s ORM layer
- Writing tests for controllers, services, and models
- Applying security best practices within Loco’s architecture

## Running the Project

```bash
cargo build
cargo loco run

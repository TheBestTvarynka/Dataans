# Overview

Dataans is a desktop note-taking application written in Rust using Tauri and Leptos.

The project consists of the following components:
* `crates/web-api-types` crate contains only types definitions that are used by the `web-server` and the desktop application.
* `crates/web-server` crate implements a simple web server for user data synchronization between application instances.
* `dataans/common` crate defined shared types between Tauri backend and Tauri frontend (Leptos).
* `dataans/src-tauri` crate is a Tauri backend implementation.
* `dataans/` crate is a Tauri frontend implementation using the Leptos web-framework.

Read crate's `AGENTS.md` file before editing it.

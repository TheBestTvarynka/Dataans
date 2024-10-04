
# Technical decisions explained

## Why [Tauri](https://tauri.app/)?

The [Tauri](https://tauri.app/) was chosen based on the following criteria:

* **Desktop app**. The app should be a desktop one.
* **Cross-platform**: at least Windows and Linux should be supported.
* Rust.
* Easy to build, package, and distribute.
* Fast.
* Secure.

## Backend

### Overview

* [Rust](https://www.rust-lang.org/).
* [PoloDb](https://www.polodb.org/).
* [tracing](https://docs.rs/tracing/).

### Details

Backend language: [Rust](https://www.rust-lang.org/).

#### Storage

The [PoloDb](https://www.polodb.org/) is used for the local data storage. [Why](https://www.polodb.org/docs/):

> PoloDB aims to offer a modern alternative to SQLite, which is currently the almost exclusive option for client-side data storage. Although SQLite is an old and stable software, it lacks some modern features. That's why we developed PoloDB, which is NoSQL, supports multi-threading and multi-sessions, and retains the embedded and lightweight features of SQLite.

This is why the PoloDb was chosen.

## Frontend

### Overview

* [Leptos](https://leptos.dev/).
* [log](https://docs.rs/log/) + [wasm-logger](https://docs.rs/wasm-logger/).

### Details

The chosen frontend framework is [Leptos](https://leptos.dev/). Why:

* Rust.
* Fast.
* Easy to write and maintain projects.
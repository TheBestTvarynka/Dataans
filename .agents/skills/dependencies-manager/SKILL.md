---
name: dependencies-manager
description: Manage Rust dependencies. Use this skill to add, edit, remove, or update dependencies.
---

* Specify the dependency version explicitly without a patch number:
  ```toml
  # Good:
  base64 = "0.23"

  # Bad:
  base64 = "0.23.1"
  ```
* Group dependencies. Groups are separated by an empty line.
  Each group has a title in the `.toml` file at the start.
  Every dependency should be in one of the following groups, in this order:
  1. `Logging`: dependencies related to logging, tracing, and diagnostics.
  2. `Db` or `Storage`: dependencies related to database access and management.
  3. `Web`: dependencies related to web frameworks, HTTP clients, web servers, and web applications.
  4. `Crypto`: dependencies related to cryptography, encryption, and security.
  5. `Utils`: serialization, deserialization, formatting, time, and other utility libraries.
  6. `Runtime`: dependencies related to `tauri`, async runtimes, task scheduling, and concurrency.
* Do not create a new group for a dependency.
  If a dependency does not fit into any of the existing groups, ask the maintainer to add a new group.
# Overview

`web-api-types` is a crate that contains only types definitions that are used by the `web-server` and the desktop application.

## New type pattern

The `web-api-types` crate strictly follows new-type pattern:

* Every public type must be wrapped in a new type:
  ```rust
  // Bad:
  pub type OperationId = uuid::Uuid;

  // Good:
  pub struct OperationId(uuid::Uuid);
  ```
* Use the `derive_more` crate for implementing conversions between inner and wrapper types.
* Inner types that do not have invariants may be public.
* Serialize time and date using `rfc3339` format only.
* The crate has a `rocket` web-server support via the `server` feature.
  `rocket`-related trait implementations must be feature-gated so the desktop client does not pull web-server dependencies.

## `serde` support

Every type must support serialization and deserialization using the `serde` library.
Use `camelCase` only naming style for serialized struct fields.


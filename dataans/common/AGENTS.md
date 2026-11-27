# Overview

The `common` crate defined shared types between Tauri backend and Tauri frontend (Leptos).

* Use new-type pattern where applicable to avoid misusage of values.
* Use the `derive-more` crate to implement wrapper<->inner types conversions.

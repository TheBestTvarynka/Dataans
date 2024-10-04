
# Technical decisions explained

## Why [Tauri](https://tauri.app/)?

In short, [Tauri](https://tauri.app/) was chosen based on the following criteria:

* **Desktop app**. The app should be a desktop one.
* **Cross-platform**: at least Windows and Linux should be supported.
* Rust.
* Easy to build, package, and distribute.
* Fast.
* Secure.

But it is not a comprehensive answer. There are a lot of ways how you can write desktop apps. And all of them suck.

**Native desktop app?** Then we'll need to write the UI components separately for all supported platforms. Moreover, writing native UI on Linux and Windows is a pain. We don't want and don't have time for that.

**[Qt](https://www.qt.io/)?**. No, we don't want to deal with C++. **Qt bindings?** Usually, they are pretty limited and, eventually, you will be forced to return to the original `Qt` to implement more advanced features. Another Qt disadvantage, in this case, is complexity and time-consuming. 

**[Electron](https://www.electronjs.org/)?**. Nope, thank you. It's too heavy, and slow, and we have more pleasure languages than JS/TS.

[Tauri](https://tauri.app/) is a perfect choice. And no, **Tauri is not the same as Electron** despite they have a lot in common.

So, yes, we still have some overhead and not the best performance ever, but it's easy to develop and maintain the app, we have modern tooling from the `Rust` ecosystem, advanced type system, and memory safety. It's the best option we currently have for writing desktop apps.

If to know real word number (such as build times, bundle sizes, memory usage, ), then follow this link: [github.com/Elanis/web-to-desktop-framework-comparison](https://github.com/Elanis/web-to-desktop-framework-comparison).

## Backend

### Overview

* Main programming language: [Rust](https://www.rust-lang.org/).
* Storage: [PoloDb](https://www.polodb.org/).
* Logging: [tracing](https://docs.rs/tracing/).

### Details

Backend language: [Rust](https://www.rust-lang.org/).

#### Storage

The [PoloDb](https://www.polodb.org/) is used for the local data storage. [Why](https://www.polodb.org/docs/):

> PoloDB aims to offer a modern alternative to SQLite, which is currently the almost exclusive option for client-side data storage. Although SQLite is an old and stable software, it lacks some modern features. That's why we developed PoloDB, which is NoSQL, supports multi-threading and multi-sessions, and retains the embedded and lightweight features of SQLite.

This is why the PoloDb was chosen.

## Frontend

### Overview

* Frontend framework: [Leptos](https://leptos.dev/).
* Logging: [log](https://docs.rs/log/) + [wasm-logger](https://docs.rs/wasm-logger/).

### Details

The chosen frontend framework is [Leptos](https://leptos.dev/). Why:

* Rust.
* Fast.
* Easy to write and maintain projects.
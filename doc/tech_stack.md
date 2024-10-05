
# Technical decisions explained

This documents is aimed to list technologies used in this project and explain why they were chosen.

## Why [`Tauri`](https://tauri.app/)?

In short, [`Tauri`](https://tauri.app/) was chosen based on the following criteria:

* **Desktop app**. The app should be a desktop one.
* **Cross-platform**: at least Windows and Linux should be supported.
* Rust.
* Easy to build, package, and distribute.
* Fast.
* Secure.

But it is not a comprehensive answer. There are a lot of ways how you can write desktop apps. And all of them suck.

**Native desktop app?** Then we'll need to write the UI components separately for all supported platforms. Moreover, writing native UI on Linux and Windows is a pain. We don't want and don't have time for that.

**[`Qt`](https://www.qt.io/)?** No, we don't want to deal with C++. **`Qt` bindings?** Usually, they are pretty limited and, eventually, you will be forced to return to the original `Qt` to implement more advanced features. Another Qt disadvantage, in this case, is complexity and time-consuming. 

**[`Electron`](https://www.electronjs.org/)?** Nope, thank you. It's too heavy, and slow, and we have more pleasure languages than JS/TS.

[`Tauri`](https://tauri.app/) is a perfect choice. And no, **Tauri is not the same as Electron** despite they have a lot in common.

> `Tauri` uses `Rust` as a native layer instead of JavaScript and web technologies, which results in lower memory usage and CPU usage compared to `Electron`. Additionally, Tauri is also designed to be more lightweight overall, which means that it has less overhead and a smaller binary size than `Electron`.

> Tauri apps have access to more system-level APIs than `Electron` apps, because of the use of rust.

> Tauri is built with security in mind and aims to be more secure than `Electron` by using a Rust-based native layer instead of a JavaScript-based layer.

> Tauri apps have smaller binary sizes than `Electron` apps because it’s using rust instead of javascript and other web technologies.

src: [`Electron` vs `Tauri`](https://www.coditation.com/blog/electron-vs-tauri).

So, yes, we still have some overhead and not the best performance ever, but it's easy to develop and maintain the app, we have modern tooling from the `Rust` ecosystem, advanced type system, and memory safety. It's the best option we currently have for writing desktop apps.

If you want to know real word numbers (such as build times, bundle sizes, memory usage, etc), then follow this link: [github.com/Elanis/web-to-desktop-framework-comparison](https://github.com/Elanis/web-to-desktop-framework-comparison).

The desktop app is divided into two main parts: backend and frontend.

## Desktop app backend

### Overview

* Main programming language: [`Rust`](https://www.rust-lang.org/).
* Storage: [`PoloDb`](https://www.polodb.org/).
* Logging: [`tracing`](https://docs.rs/tracing/).

### Storage

The [`PoloDb`](https://www.polodb.org/) is used for the local data storage. [Why](https://www.polodb.org/docs/):

> PoloDB aims to offer a modern alternative to SQLite, which is currently the almost exclusive option for client-side data storage. Although SQLite is an old and stable software, it lacks some modern features. That's why we developed PoloDB, which is NoSQL, supports multi-threading and multi-sessions, and retains the embedded and lightweight features of SQLite.

This is why the `PoloDb` was chosen. The `PoloDb` functionality is enough for the current app requirements.

## Desktop app frontend

### Overview

* Frontend framework: [`Leptos`](https://leptos.dev/).
* Logging: [`log`](https://docs.rs/log/) + [`wasm-logger`](https://docs.rs/wasm-logger/).

### Why [`Leptos`](https://leptos.dev/)?

The chosen frontend framework is [`Leptos`](https://leptos.dev/). Why:

* Rust.
* Fast.
* Easy to write and maintain projects.
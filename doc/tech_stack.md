
# Technical decisions explained

This documents aims to list technologies used in this project and explain why they were chosen.

## Why [`Tauri`](https://tauri.app/)?

In short, [`Tauri`](https://tauri.app/) was chosen based on the following criteria:

* **Desktop app**. The app should be a desktop one.
* **Cross-platform**: at least Windows and Linux should be supported.
* `Rust`.
* Easy to build, package, and distribute.
* Fast.
* Secure.

But it is not a comprehensive answer. There are a lot of ways how you can write desktop apps. And all of them suck.

**[`Qt`](https://www.qt.io/)?** No, we don't want to deal with C++. **`Qt` bindings?** Usually, they are pretty limited and, eventually, you will be forced to return to the original `Qt` to implement more advanced features. Another `Qt` disadvantage, in this case, is complexity and time-consuming. 

**[`Electron`](https://www.electronjs.org/)?** Nope, thank you. It's too heavy, and slow, and we have more pleasure languages than `JS`/`TS`.

[`Tauri`](https://tauri.app/) is a perfect choice. And no, **Tauri is not the same as Electron** despite they have a lot in common.

> `Tauri` uses `Rust` as a native layer instead of JavaScript and web technologies, which results in lower memory usage and CPU usage compared to `Electron`. Additionally, Tauri is also designed to be more lightweight overall, which means that it has less overhead and a smaller binary size than `Electron`.

> `Tauri` apps have access to more system-level APIs than `Electron` apps, because of the use of `Rust`.

> `Tauri` is built with security in mind and aims to be more secure than `Electron` by using a `Rust`-based native layer instead of a `JavaScript`-based layer.

> `Tauri` apps have smaller binary sizes than `Electron` apps because it’s using rust instead of `JavaScript` and other web technologies.

src: [`Electron` vs `Tauri`](https://www.coditation.com/blog/electron-vs-tauri).

So, yes, we still have some overhead and not the best performance ever, but it's easy to develop and maintain the app, we have modern tooling from the `Rust` ecosystem, advanced type system, and memory safety. It's the best option we currently have for writing desktop apps.

If you want to know real word numbers (such as build times, bundle sizes, memory usage, etc), then follow this link: [github.com/Elanis/web-to-desktop-framework-comparison](https://github.com/Elanis/web-to-desktop-framework-comparison).

The desktop app is divided into two main parts: backend and frontend. And there is also sync server for user's data back up and sync.

## Desktop app backend

* Main programming language: [`Rust`](https://www.rust-lang.org/).
* Storage: [`sqlite`](https://www.sqlite.org/).
* Logging: [`tracing`](https://docs.rs/tracing/) and [`tracing-subscriber`](https://docs.rs/tracing-subscriber/).

## Desktop app frontend

* Frontend framework: [`Leptos`](https://leptos.dev/).
* Logging: [`log`](https://docs.rs/log/) + [`wasm-logger`](https://docs.rs/wasm-logger/).

### Why [`Leptos`](https://leptos.dev/)?

The chosen frontend framework is [`Leptos`](https://leptos.dev/). Why:

* `Rust`.
* Fast.
* Easy to write and maintain projects.

Keep in mind that **only Rust frontend frameworks were considered** for this project. We don't want to deal with `JS`/`TS` and want to implement everything in `Rust`. `Rust` has a lot of interesting and worth-trying [frontend frameworks](https://www.arewewebyet.org/topics/frameworks/). The `create-tauri-app` *had* (at the moment of the project creation) a [limited template list](https://tauri.app/start/create-project/), so our list of candidates was shrunk to 3 ones: [Yew](https://yew.rs/), [Leptos](https://leptos.dev/), and [Sycamore](https://sycamore-rs.netlify.app/).

All of them are popular, fast, and interesting: [`Yew` vs `Dioxus` vs `Leptos` vs `Sycamore`](https://www.reddit.com/r/rust/comments/1526qo3/comment/jsdq72u/). We chose `Leptos` because of:

> * fine-grained reactivity, with no virtual DOM overhead.
> * Entirely safe Rust.
> * Very, very good performance.

It doesn't mean that the `Leptos` is the best. We just like it more and it's enough for us :stuck_out_tongue_closed_eyes:.

## Sync server

* Main programming language: [`Rust`](https://www.rust-lang.org/).
* Web framework: [`Rocket`](https://rocket.rs/).
* Storage:
    * [`Postgres`](https://www.postgresql.org/).
    * [`Tigris`](https://www.tigrisdata.com/).
* Auth: [Cloudflare Zero Trust Access](https://www.cloudflare.com/zero-trust/products/access/).
* Deployment infrastructure: [fly.io](https://fly.io/).
* Logging: [`tracing`](https://docs.rs/tracing/) and [`tracing-subscriber`](https://docs.rs/tracing-subscriber/).
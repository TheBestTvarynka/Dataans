# Dataans

## Technology stack

[Tauri](https://tauri.app/). Tauri is a framework for building tiny, blazingly fast binaries for all major desktop platforms.

[Leptos](https://leptos.dev/). Leptos is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces.

Thus, everything starting from the back end and up to the front end is written in Rust. More about tech stack: [`tech_stack.md`](/doc/tech_stack.md).

## Installation

Note: this is a temporary solutions. One day we will have a proper flow of app installing and releases publishing.

First of all, you need to fulfill Tauri Prerequisites first: https://v1.tauri.app/v1/guides/getting-started/prerequisites.

```bash
git clone https://github.com/TheBestTvarynka/Dataans.git
cd Dataans/dataans
cargo tauri build
mkdir -p ~/.local/share/com.tbt.dataans/configs/
cp src-tauri/resources/configs/* ~/.local/share/com.tbt.dataans/configs/
```

## Development

0. Tauri Prerequisites: https://v1.tauri.app/v1/guides/getting-started/prerequisites.
1. Run:
```bash
git clone https://github.com/TheBestTvarynka/Dataans.git
cd Dataans/dataans

# Optional: logging
export DATAANS_LOG=dataans=trace

cargo tauri dev
```

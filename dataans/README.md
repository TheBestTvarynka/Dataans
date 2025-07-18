# Dataans

## Technology stack

[Tauri](https://tauri.app/). Tauri is a framework for building tiny, blazingly fast binaries for all major desktop platforms.

[Leptos](https://leptos.dev/). Leptos is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces.

Thus, everything starting from the back end and up to the front end is written in Rust. More about tech stack: [`tech_stack.md`](/doc/tech_stack.md).

## Installation

> [!NOTE]
> This is a temporary solution. One day we will have a proper flow of app installation and release publishing.

1. Tauri Prerequisites: https://v2.tauri.app/start/prerequisites/.
2. Install needed cli tools:
   ```bash
   rustup target add wasm32-unknown-unknown
   # https://v2.tauri.app/reference/cli/
   cargo install tauri-cli --version "^2.0.0" --locked
   # https://trunkrs.dev/#install
   cargo install --locked trunk
   ```
3. Clone the repo:
   ```bash
   git clone https://github.com/TheBestTvarynka/Dataans.git
   cd Dataans/dataans
   ```
4. Build:
   ```bash
   # Build the app
   cargo tauri build
   ```
5. Configure:
   ```bash
   # Create directory for the app data
   mkdir -p ~/.local/share/com.tbt.dataans/configs/
   # Initialize default app configs
   cp src-tauri/resources/configs/* ~/.local/share/com.tbt.dataans/configs/
   ```
6. Run the app. You have two options for how to run it:
   1. Install the app using the installation package and run from the application launcher/start menu. The installation package is located in `target/release/bundle`.
   2. Do not install the app globally but run the app executable file. The app executable file is the following file: `target/release/dataans`.
7. _(Optional)._ Additional app configuration:
   * You can set the logging level using the `DATAANS_LOG` environment variable. For example, `DATAANS_LOG=trace`.

## Development

The steps are almost the same as in the previous chapter.

1. Tauri Prerequisites: https://v2.tauri.app/start/prerequisites/.
2. Install needed cli tools:
   ```bash
   # https://v2.tauri.app/reference/cli/
   cargo install tauri-cli --version "^2.0.0" --locked
   # https://trunkrs.dev/#install
   cargo install --locked trunk
   ```
3. Clone the repo:
   ```bash
   git clone https://github.com/TheBestTvarynka/Dataans.git
   cd Dataans/dataans
   ```
4. Configure:
   ```bash
   # Create directory for the app data
   mkdir -p ~/.local/share/com.tbt.dataans/configs/
   # Initialize default app configs
   cp src-tauri/resources/configs/* ~/.local/share/com.tbt.dataans/configs/

   # Optinoal: logging
   export DATAANS_LOG=dataans=trace

   # Optional: if you want, you can overwrite the local database location file with the environment variable:
   export BATABASE_URL=<path/to/file.sqlite>
   ```
5. Run the development server:
   ```bash
   # Run the development server
   cargo tauri dev
   ```

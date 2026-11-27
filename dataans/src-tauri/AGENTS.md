# Overview

The `dataans` crate is a core of the application.
It implements [Tauri] backend and contains all application logic.

## Tauri commands

Desktop application frontend and backend communicate using Tauri commands.

* [Calling the Frontend from Rust](https://v2.tauri.app/develop/calling-frontend/).
* [Calling Rust from the Frontend](https://v2.tauri.app/develop/calling-rust/).

There two types of Tauri commands:

* Dataans application core behavior commands must be defined inside the `dataans` Tauri plugin.
  This plugin is located in `src/dataans`.
  It contains commands for notes, spaces, synchronization, and more.
* All other commands must be located outside the `dataans` Tauri plugin.
  For example, window management, code block formatting, config file reading, etc.

Do not forget to edit the project [capabilities](./capabilities/) after adding or removing Tauri commands.

## Sync

Read the [`sync_server.md`](../../doc/sync_server.md) document to get synchronization and encryption algorithms description.

[Tauri](https://v2.tauri.app/)

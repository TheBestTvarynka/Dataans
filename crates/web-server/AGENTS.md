# Overview

The `web-server` crate implements a simple web server for user data synchronization between application instances.
The synchronization specification is located in [`sync_server.md`][1].

## Architecture

The web-server follows a layered architecture approach.

* The API layer, the top most layer, contains web API routes and endpoints definitions.
  All of them are located in the `src/routes/` directory.
* The service layer, the middle one, contains main web-server logic implementation.
  All services are located in the `src/services/` directory.
* The DB layer, the lowest one, contains the database access layer implementation.
  It is responsible for writing to the database and reading from it.
  The database layer implementation is located in the `src/db/` directory.

## Security

All user data is E2E encrypted.
The `web-server` is unable to decrypt it.
The `web-server` receives and responds only with encrypted blobs.

The authentication scheme is explained in [`sync_server.md`][1].

### Files

The `web-server` is able to sync user's actions but it is also able to serve files.
All use files are encrypted and the server cannot decrypt them.
For the `web-server` any use file is just name and a bunch of bytes.

[1](../../doc/sync_server.md)


# Sync/back up server

The Dataans app supports user's data back up to the remote server. The same server is used for the data synchronization between devices.
Let me answer your next question. Nope, the P2P sync between devices is not implemented (at least yet).

In the app terminology, the back up and sync servers are the same thing.
If you use Dataans only on one device (or many devices but different purposes), then the remote server is the back up server.
If you use Dataans on many devices and want them to share the same state, then the remote server is the sync server.
In any case, sync and back up servers are the same server instance. It is just a small clarification, so you do not get confused.

# How it works

## General overview

The user has some local database state. Alongside user's data, the app also tracks all user operations like note creation, updating, space deletion, etc. During the sync process all these operations are synchronized.

The server knows nothing about the content of these operations. Because the app encrypts the data before upload.
The server knows only the operation id, but it is not beneficial because all ids are randomly generated UUIDs.

**The one server can contain data for one user. The server is not designed for multi-user usage.**

## Encryption

The app needs two things to generate the encryption key:

1. The user's password.
2. The passphrase (salt).

The user is responsible for storing the password securely.

The passphrase is automatically generated during the first sign in and stored in the `profile.json` file in the app data directory.
The user must enter the same passphrase on next sign ins. If the user does not want the app to generate the passphrase, then they can enter it manually during the first sign in.

The app uses [PBKDF](https://en.wikipedia.org/wiki/PBKDF2) to generate the encryption key. Here is a generation scheme:

```rust
// Pseudocode
let password = sha256(password);
let salt = sha256(passphrase);
let iteration_count = 1_200_000;

let key = pbkdf2(password, salt, iteration_count);
```

## Auth

The best way to implement auth is not to implement it. So, [Cloudflare Zero Trust Access](https://www.cloudflare.com/zero-trust/products/access/) has been chosen as the auth provider for the server.
It works very convenient and simple:

1. Cloudflare is configured to determine who can access the web server (e.g., by an allowed email list).
   When someone tries to access the server endpoint, Cloudflare catches the request and redirects to the authorization page, if needed.
2. The server checks for Cloudflare's header and validates it using Cloudflare's public key. See more here: [Cloudflare Zero Trust/Identity/Authorization cookie/Validate JWTs](https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/validating-json/).

If you want to support any other authorization method, you need to implement it :upside_down_face:.

## Sync


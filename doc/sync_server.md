
# Sync/backup server

# What? Why?

The Dataans app has followed a _local first_ approach since the very beginning of its existence. After some time, I started to use the app on many devices and inside my Windows/Linux VMs.
I caught myself on a thought that it would be good to be able to transfer the data from one device to another.
After some considerations, I understood that I want the multi-device synchronization feature.

Decentralized Internet, P2P networks, etc, are good. But the data synchronization feature relies on the central sync (backup) server.
The data synchronization using the P2P communication may be implemented in the future. It depends on my needs.

# Intro

The Dataans app supports user's data back up to the remote server. The same server is used for the data synchronization between devices.

In the app terminology, the back up and sync servers are the same thing.
If you use Dataans only on one device (or many devices that do not share the data between each other), then the remote server is the back up server.
If you use Dataans on many devices and want them to share the same state, then the remote server is the sync server.
In any case, sync and backup servers are the same server instance. It is just a small clarification, so you do not get confused.

# How it works

## General overview

The user has some local database state. Alongside users' data, the app also tracks all user operations like note creation, updating, space deletion, etc. During the sync process, all these operations are synchronized.

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

Before uploading, all data is encrypted using the following encryption scheme:

```rust
// Pseudocode
let nonce = random();

let cipher_text = aes_gcm.encrypt(key, nonce, plaintext);
let checksum = hmac_sha256(key, nonce + plaintext);

let result = nonce + plaintext + checksum;
```

The same for decryption, but in reverse:

```rust
// Pseudocode
let (nonce, cipher) = result.split_at(NONCE_LEN);
let (cipher_text, checksum) = cipher.split_at(cipher.len() - HMAC_LEN);

let plaintext = aes_gcm.decrypt(key, nonce, cipher_text);
let calculated_checksum = hmac_sha256(key, nonce + plaintext);

if checksum != calculated_checksum {
   return Err("data is altered");
}

Ok(plaintext)
```

## Sync

Optionally (if configured), the user can configure data synchronization.

If it is configured, then the app can synchronize all users' data (spaces, notes, files, etc.) with the remote server. Also, the user can sign in on multiple devices using the same credentials and sync the data. Multi-device data synchronization can be achieved this way.

In order to start the sync process, the user needs to do two things:

1. Deploy the web-server (the sync-server).
2. Sign in using the app setting page.

### Auth

The best way to implement auth is not to implement it. So, [Cloudflare Zero Trust Access](https://www.cloudflare.com/zero-trust/products/access/) has been chosen as the auth provider for the server.
It works very conveniently and simply:

1. Cloudflare is configured to determine who can access the web server (e.g., by an allowed email list).
   When someone tries to access the server endpoint, Cloudflare catches the request and redirects to the authorization page, if needed.
2. The server checks for Cloudflare's header and validates it using Cloudflare's public key. See more here: [Cloudflare Zero Trust/Identity/Authorization cookie/Validate JWTs](https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/validating-json/).

If you want to support any other authorization method, you need to implement it :upside_down_face:.

### Sync algorithm

The app has some local state: spaces, notes, images, files, etc. All files (including images) are stored directly on the disk in the `files` directory.
All other data is stored in the SQLite database.

Additionally, the app also stores in a separate table all user's actions that alters the local state in any way: space creation, note editing, adding a new file, etc. Instead of syncing the app's full state, the app syncs the operations list (files are handled separately).

The naive approach would be to request all operations the server has, and then compare them to local ones, and find the difference. But there can be a lot of operations. So, there is a small optimization: _synchronization blocks_ (or _sync blocks_, or just _blocks_).

We do now want to request all operations and compare them to local ones. We would like to discard the common operations. Let's sort app operations by timestamp and split into blocks. Let's say into blocks of 256 notes. Then, we calculate block hash:

```rust
// Pseudocode
let block_hash = hash(hash(notes[0]) | ... | hash(notes[255]));
```

As a result, we will have a list of block hashes. We do this procedure on the server-side (the sync server) and on the client-side (the app). 
Then the server's blocks hash list is transferred to the local app. The app compared these two lists. The same operations will result in the same block hashes.
So, we can discard operations that belong to blocks with the same hashes. If either side has some operation that the other side does not have, then all consecutive blocks will have different notes, and, in turn, different hash values.

After that, the app requests the server's notes starting from the end of the last discarded block. The same for local operations: the app selects local operation starting from the last discarded block.

As a result, the app has two operation lists: local and remote. Both lists are sorted by timestamp. The app finds common operations for both lists.
Then, these common operations are eliminated from local and remote operations lists. After that, we will have two lists with unique operations.

The rest is an easy job. The app applies remote operations on a local database and uploads local operations.

### Conflict resolution

Every object (like a note, a space, or a file) has creation and updating timestamps. During the operation applying, the object with the latest timestamp wins (i.e., last write wins).

During operations uploading, the sync server accepts operations and inserts them into the operations table. Nothing more.

### Files sync

Files are not stored in the SQLite database but directly on the disk.
When the sync process starts, the app iterates over all files registered in the local database and determines which ones need to be uploaded/downloaded.
There are possibly the following cases:

1. If the `is_uploaded` attribute is `true` and the file is present locally, then the app will do nothing.
2. If the `is_uploaded` attribute is `true` and the file is not present locally, then the app will try to download the file from the sync server.
1. If the `is_uploaded` attribute is `false` and the file is present locally, then the app will try to upload it on the sync server.
1. If the `is_uploaded` attribute is `false` and file does not present locally, then the app will log an error.
   Ideally, such situation should never exist. In general case, it means that this is a bug or someone/something (not the app) edited the local app database.

The app may discover new files during remote operation applying process. You should not worry about it. The app automatically will try to download them immediately.

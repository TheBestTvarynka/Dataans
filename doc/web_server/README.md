This document aimed to describe the server backup, synchronization functionality, users identity system. It contains high-level explanations, diagrams, motivation, reason, and purpose for all technical decisions.

> [!WARNING]
> It is an initial version of the document and the implementation hasn't started yet. The content may be changed in the future.
> All comments, feedback, questions, and proposals are highly welcome.

# Motivation

Currently (23.11.2024), all notes, spaces, files, and all other data are saved only on the user's computer without any synchronization between devices or even between client-server.

As a user, I want to be able to synchronize my data between devices and to have online backups on the web server. Moreover, it shouldn't block me from using the app offline.

# Goals

- To be able to sync data between server and client (local app).
- To be able to have multiple devices that will sync the same data (data that belong to the same user).
- Encryption of the data located on the server. All data on the server should be encrypted and the server should know nothing about the user's identity or user's data content.

# Design

## Encryption

The web server serves as the user's data backup storage. It doesn't know about the data content or the user's identity. All data is encrypted using a strong encryption algorithm and the encryption key is known only to the user.

All security design is designed with the following statements in mind:

* Never trust any server.
* Trust the client's computer. So, the local DB is not encrypted and can be read by any app on the client's computer.

All data is encrypted using the [AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) [GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode) algorithm. The random AES block (16-byte) is prepended to the plaintext data to make the encrypted non-deterministic (e.g. the same data encrypted with the same key will result in the different cipher texts).
Additionally, the [HMAC](https://en.wikipedia.org/wiki/HMAC) SHA256 checksum is calculated over the plaintext data. HMAC is used to ensure data integrity.

The summarized encryption scheme looks like this:

```rust
// Pseudocode
fn encrypt(key: &[u8], plain_text: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let confounder: [u8; AES_BLOCK_SIZE] = rand.gen();

    let plain_text = confounder + plain_text;

    let cipher_text = aes_gcm_256.encrypt(key, plain_text);
    let checksum = hmac_sha_256.hmac(key, plain_text);
    
    (cipher_text, checksum)
}
```

The same principle with data decryption but reversed:

```rust
// Pseudocode
fn decrypt(key: &[u8], cipher_text: &[u8], checksum: &[u8]) -> Vec<u8> {
    let plain_text = aes_gcm_256.decrypt(key, cipher_text);

    hmac_sha_256.verify(key, plain_text, checksum).expect("Data has been corrupted");

    // Remove confounder block:
    plain_text[AES_BLOCK_SIZE..].to_vec()
}
```

The server does not and can not decrypt the user's data. Only the user (client) can do it because only the client can derive the encryption key.

## Key types and their derivation

The key derivation process consists of a few stages and needs two components: a secret key and a user's password.

### Secret key

The secret key is a piece of random bytes generated during the user's sign-up process. It is an additional security. Even if the password is compromised, the attacker will not be able to decrypt the data. The attacker needs a password and secret key to decrypt the cipher text.

If the user needs to add another device to their account, then it will need to type the secret key on the new device.

The secret key is stored in plain text on the computer. We trust the user's computer by design.

### Password

The password is created by the user during the sign-up process. The user is responsible for keeping the password safe. The app never stores the user's password.

### Encryption key derivation

The encryption key derivation process is shown below:

```rust
// Pseudocode
fn derive_encryption_key(secret_key: &[u8], password: &[u8], user_id: &[u8]) -> Vec<u8> {
    let k1 = pbkdf2(password, user_id /* salt */, 650_000 /* number of iterations */);
    let k2 = hkdf(secret_key, user_id /* salt */, k1.len());

    xor(k1, k2);
}
```

## Auth

### Sign-up

The user types the invitation token, password, and username during the sign-up process.

* `Invitation token`. The server has limited capacity and the app is no-profit. So, the number of users is also limited. You need to have an invitation token to sign up.
* `Username`. It can be any friendly name. The server doesn't share any data with anyone. Moreover, the server stores the hash of the username. The only reason why the user needs a username is to simplify the sign-in process on new devices.
* `Password`.

That's all. The server will create a new session and the sync process will begin right after the successful sign-up.

### Sign-in

You can install the app on any supported device and sign in. The sync process will begin right after the successful sign-in. To sign-in user needs to provide the following:

* `Username`.
* `Password`.
* [`Secret key`](#secret-key). This key is automatically generated during the sign-up process. The user can use the app on any other logged-in device to show the secret key.

> [!CAUTION]
> If the user loses their secret key (fs corruption, lost the laptop, etc), then they will not be able to decrypt the data. If you want, you can also put the secret key in your password manager alongside the password.

### 2FA

2FA is possible but optional. The only supported 2FA method is the Authenticator App. The user can ask the app to generate the QR code and recovery codes.

2FA secret and recovery codes are stored on the server side and encrypted using the server's encryption key.

2FA does not make the encryption of the data stronger. It makes the auth process stronger and better. In other words, the [secret key](#secret-key) is *a second factor* for data encryption like the authenticator app is the second factor for auth. The attacker with the password is unable to decrypt the data without a secret key and to sign-in without 2FA.

## Data synchronization

# Cursed Stories I have experienced when building/using Dataans

## 06.08.2026

We all know about the different file lines endings ([Difference between CR LF, LF and CR line break types](https://stackoverflow.com/a/1552775)).
Usually, I have no provlems with them. I just commit files as they are and do not care.
Even more, CR became a standart and `git` itself tries to force us to always use CR.
Sometimes you can see messages from `git` like this:

> LF will be replaced by CRLF

Surprisinly, I still faced a problem with it.
Despite all the progress in 2026, you still need to know and care about line endings.

I opened my macOS machine, cloned the repo, built the app, and run it...
Oh, wait. It did not run. The app paniced with the following message:

> thread 'main' (33909647) panicked at dataans/src-tauri/src/dataans/mod.rs:66:43:
> Failed to run migrations: VersionMismatch(20241210221629)

What?
I did not change any files! I just cloned the repo!
Spoiler: the problem was in the `database.sql` file which I copied from the Windows machine and in the `git` itself.

Let's start from the db file.
Why did I copy it?
Because I have a bug in the sync mechanism which I have not fixed yet :upside_down_face:.
My plan was to copy the db file and continue using the app as usual.
It sounds horrible but that day it was okay enough for me.

Next, why did the migration command fail?
Apparently, it is because of the checksum mistmatch.
`sqlx` calculates migration file checksum and compares it with the checksum in the migration table.
In my case, these checksums were different and `sqlx` reported an error.

Now let's talk about line endings.
Windows uses CRLF line endings.
Misgration files have CRLF line endings.
When the app run migrations on Windows, the `sqlx` calculated migration files hashes and wrote them into the migrations table.

Then I switched to macOS where migration files have LF line endings.
So, the calculates checksum by `sqlx` is different from the one written in the db.
Voila, migration error.

Did you notice anything weird?
In both cases I did no modifications and got different checksums.
Is is because `git` performs automatic files conversion when you clone the repo, add files to index, pull changes, checkout,  etc.

Wait! The default value for `core.autocrlf` is `false` and I did not change it.
Why does git do automatic line ending conversion?
He he. Because when you install `git` on the Windows machine, the isnstalled asks you how to deal with line endings.
Me and probably you usually do not read that and just click "Next".

### Summary

I got the checmsum mistmatch error because migration file hash on Windows and macOS is different.
The hash is different because all files in the repo are automatically converted by `git` to have CRLF enings.
`git` automatically convers line endings because I told it to do so during `git` installation and forgot about it.

### Conclusions



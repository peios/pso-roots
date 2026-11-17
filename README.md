# pso-roots

The public root keys of Peios Signed Objects (PSO). A root signs top-level
certificates and nothing else, and a verifier accepts a root only from its
own build. This repository is where those builds get them.

Each file in `roots/` is one root's public key, in the form `pso root`
writes:

```text
pso-public-key ml-dsa-87 <key in hexadecimal>
```

## The roots

Both roots are **provisional**. They are for building and testing before
the public release, and will be replaced before it. Nothing that depends
on them is a production build, and every top-level they sign stops
verifying when they are replaced.

| File | Type | Key hash |
|---|---|---|
| `roots/peios-root-1.pub` | ML-DSA-87 | `f37aa6246e0370d6ef5b0f040e0cb5a3cbebf91488c41c9044b240253a79a328381353a6e9d4a8155cd185120bbf768f45a9e640757cda1c446da5905bb3ad41` |
| `roots/peios-root-2.pub` | ML-DSA-87 | `c5e0034559e7566bab7be171ca65d413d258f369c9d78ee78ee1ed629b8116a12c765bf15eed58f5eca5e21d91d8183bc94eb43aa44e7384a89aa3b7b0c4a726` |

A root's key hash is the SHA-512 of its key type byte followed by its key.
Compare the hashes a build embedded with this table.

## Rules

- Only public keys go here. A secret key never does.
- Only production roots go here. Keys for development and for tests stay
  in the code that uses them, so that none can reach a release build.
- A build pins this repository by commit and content, as `pekit.lock`
  does. A change here reaches a build only when that build's lock is
  updated.
- A root is added or retired in a commit of its own that says which and
  why.

## Using them

A verifier embeds every file in `roots/` at build time. The Peios kernel
and ppk are the verifiers that do.

A verifier written in Rust depends on this repository as a crate, pinned
by commit, and reads `pso_roots::ROOTS`. A root added to `roots/` is also
added to `src/lib.rs`, and a test fails if the two differ.

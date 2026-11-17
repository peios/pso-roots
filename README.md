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

No root has been published.

Each root is listed here by its key hash when it is added, so that what a
build embedded can be compared with what this repository holds. `pso key
public` prints a key's hash.

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

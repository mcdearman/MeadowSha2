# sha2

The SHA-2 hash functions of
[FIPS 180-4](https://csrc.nist.gov/pubs/fips/180-4/upd1/final) (SHA-224,
SHA-256, SHA-384, SHA-512, SHA-512/224 and SHA-512/256), for
[Meadow](https://github.com/mcdearman/meadow).

This package is a port of Rust's [`sha2`](https://github.com/RustCrypto/hashes)
0.10.9.

## Install

```sh
meadow add mcdearman/Sha2
```

## Use

```meadow
use Sha2 (sha256, digestString, toHex, Sha512t256)

def main =
  ( toHex (sha256 (stringToBytes "abc")),
    -- "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    toHex (digestString Sha512t256 "abc")
  )
```

| function | |
|---|---|
| `sha224`, `sha256`, `sha384`, `sha512` | the digest of some bytes, as bytes |
| `digest alg bytes`, `digestString alg s` | the same for any `Algorithm`: `Sha224`, `Sha256`, `Sha384`, `Sha512`, `Sha512t224` or `Sha512t256` |
| `new alg`, `update hasher bytes`, `finalize hasher` | hashing a piece at a time |
| `outputSize alg` | the digest length in bytes |
| `toHex bytes` | lower-case hex, the usual way to show a digest |

`Sha512t224` and `Sha512t256` are the crate's `Sha512_224` and `Sha512_256`.
Meadow constructor names can't contain `_`.

**Do not use SHA-2 alone to hash passwords.** It is built to be fast, which
makes guessing passwords fast too. Use a password-hashing function instead.

## How it's made

`src/Sha2.mw` translates the crate's portable implementation. The round
constants and initial states in `src/Tables.mw` are generated from the crate.
**`src/Cases.mw`** is generated test data: 404 messages, one of every length up
to 400 bytes (which covers every padding case) plus a few long ones. Each is
hashed by all six functions, both whole and in two pieces, and `meadow test`
checks every digest against the crate's. Run `scripts/generate.sh` to
regenerate; it needs a Rust toolchain.

## Licence

Dual-licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your
option, like the crate. See [COPYRIGHT](COPYRIGHT).

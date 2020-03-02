# Changelog

All notable changes to surf will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://book.async.rs/overview/stability-guarantees.html).

## [Unreleased]

## [2.0.0-alpha.0] - 2020-03-02

This is an alpha release in preparation of 2.0.0, so you can start using Surf with stable `futures`. There may be significant breaking changes before the final 2.0 release. Until thin, we recommend pinning to the particular alpha:

```toml
[dependencies]
surf = "= 2.0.0-alpha.0"
```

### Added

- Transcode responses from non-UTF8 charsets.

### Changed

- Updated to use stable `futures`.
- Build on top of [`http-client`](https://github.com/http-rs/http-client).
- Set up an MSRV in CI.
- Stop hitting the network when running tests.
- Reduce copies when parsing URLs.

### Fixed

- Fix syntax errors in the README.md example.
- Fix links in CHANGELOG.md after the org move in 1.0.3.

## [1.0.3] - 2019-11-07

### Changed

- Migrated the project from the `rustasync` organization to `http-rs`.
- Migrated CI providers from Travis CI to GitHub Actions.
- Replaced `runtime` with `async-std` in examples.
- Error context no longer discards the inner error body.
- Updated the README.md formatting.
- Updated `futures-preview` to `0.3.0-alpha.19`

## [1.0.2] - 2019-08-26

Log not kept.

## [1.0.1] - 2019-08-15

Log not kept.


## [1.0.0] - 2019-08-15

Log not kept.

[Unreleased]: https://github.com/http-rs/surf/compare/1.0.3...HEAD
[2.0.0-alpha.0]: https://github.com/http-rs/surf/compare/1.0.3...2.0.0-alpha.0
[1.0.3]: https://github.com/http-rs/surf/compare/1.0.2...1.0.3
[1.0.2]: https://github.com/http-rs/surf/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/http-rs/surf/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/http-rs/surf/compare/1.0.0

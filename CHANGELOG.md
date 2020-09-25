# Changelog

All notable changes to surf will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://book.async.rs/overview/stability-guarantees.html).

## [Unreleased]

## [2.0.0-alpha.5] - 2020-09-07

This is an alpha release in preparation of 2.0.0, so you can start using Surf with stable futures. There may be significant breaking changes before the final 2.0 release. Until thin, we recommend pinning to the particular alpha:

```toml
[dependencies]
surf = "= 2.0.0-alpha.5"
```

This alpha release notably contains much more API parity with Tide, particularly for `surf::Request`, `surf::Response`, and `surf::middleware::Middleware`. Middleware also is now implemented using [async-trait](https://crates.io/crates/async-trait). Additionally, `surf::Client` is no longer generic and now instead holds the internal `HttpClient` as a dynamic trait object.

These changes mean that surf middleware must undergo the following changes:

**Old middleware:**
```rust
impl<C: HttpClient> Middleware<C> for Logger {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, http_types::Error>> {
        Box::pin(async move {
            Ok(res)
        })
    }
}
```

**New middleware:**
```rust
#[surf::utils::async_trait]
impl Middleware for Logger {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response> {
        Ok(res)
    }
}
```

This alpha release also contains large changes to how the `surf::Request` and `surf::Client` APIs are structured, adding a `surf::RequestBuilder` which is now returned from methods such as `surf::get(...)`. Overall usage structure was kept the same where possible and reasonable, however now a `surf::Client` must be used when using middleware.

```rust
let client = surf::client()
    .with(some_middleware);

let req = surf::post(url) // Now returns a `surf::RequestBuilder`!
    .header(a_header, a_value)
    .body(a_body);
let res = client.send(req).await?;
```

### Additions
- `surf::Request` added many methods that exist in `tide::Request`.
- `surf::Response` added many methods that exist in `tide::Response`.
- `surf::http`, an export of `http_types`, similar to `tide::http`.
- `surf::middleware::Redirect`, a middleware to handle redirect status codes.
- All conversions for `Request` and `Response` between `http_types` and `surf` now exist.

### Changes
- `surf::Request` changed many methods to be like those in `tide::Request`.
- `surf::Response` changed many methods to be like those in `tide::Response`.
- Surf now uses `http-types::mime` instead of the `mime` crate.
- `TryFrom<http_types::Request> for Request` is now `From<http_types::Request> for Request`.
- `surf::Client` is no longer generic for `C: HttpClient`.
- Middleware now receives `surf::Request` and returns `Result<surf::Response, E>`, and no longer requires a generic bound.
- Middleware now uses [async-trait](https://crates.io/crates/async-trait), which is exported as `surf::utils::async_trait`.
- The logger middleware is now exported as `surf::middleware::Logger`. (Note: this middleware is used by default.)
- `surf::{method}()` e.g. `surf::get()` now returns a `surf::RequestBuilder` rather than a `surf::Request`.
  - Middleware can no longer be set for individual requests.
  - Instead, use a `surf::Client` and register middleware via `client.with(middleware)`.
  - Then, send the request from that client via `client.send()` e.g. `let res = client.send(request).await?;`.
- `surf::Client` now can set a "base url" for that client via `client.set_base_url()`.

### Fixes
- `From<http_types::Request> for Request` now properly propagates all properties.
- A cloned `surf::Client` no longer adds middleware onto its ancestor's middleware stack.
- Some feature flags are now correct.

### Internal
- Use Clippy in CI.
- Improved examples.
- Now only depends on `futures_util` rather than all of `futures`.

## [2.0.0-alpha.2] - 2020-04-29

This is an alpha release in preparation of 2.0.0, so you can start using Surf with stable `futures`. There may be significant breaking changes before the final 2.0 release. Until then, we recommend pinning to the particular alpha:

```toml
[dependencies]
surf = "= 2.0.0-alpha.2"
```

### Added
- `http_types::{Error, Result}` are re-exported as `surf::{Error, Result}` https://github.com/http-rs/surf/pull/163

### Changed
- Add http-rs logo for docs.rs. https://github.com/http-rs/surf/pull/165

### Fixed
- Removed dependencies that are no longer necessary since the switch to [`http-client`](https://github.com/http-rs/http-client) in 2.0.0-alpha.0 https://github.com/http-rs/surf/pull/164

## [2.0.0-alpha.1] - 2020-04-17

This is an alpha release in preparation of 2.0.0, so you can start using Surf with stable `futures`. There may be significant breaking changes before the final 2.0 release. Until thin, we recommend pinning to the particular alpha:

```toml
[dependencies]
surf = "= 2.0.0-alpha.1"
```

### Added
- `h1-client` backend is now available https://github.com/http-rs/surf/pull/146

### Changed

- Updated `http-client` to v2.0.0
- Switched from `hyperium/http` to [`http-types`](https://docs.rs/http-types/1.2.0/http_types/) https://github.com/http-rs/surf/pull/146

### Fixed
- Updated `mime-guess` requirement https://github.com/http-rs/surf/pull/148

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

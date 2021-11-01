# Changelog

All notable changes to surf will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://book.async.rs/overview/stability-guarantees.html).

## [Unreleased]

## [2.3.2] - 2021-11-01

### Fixes
- Fixed `Config::set_tls_config()` when using `h1-client` (`async-h1` with `async-native-tls`).
    - Previously this function was never exposed due to a faulty feature flag check.

## [2.3.1] - 2021-08-23

Fixed git base of 2.3.0

## [2.3.0] - 2021-08-23

(Yanked, faulty git base)

### Additions
- `surf::Config`, a way to configure `surf::Client`-s!
    - `Config::add_header()` - client-wide headers
    - `Config::set_base_url()` - client-wide base url
    - `Config::set_http_keep_alive()`
    - `Config::set_tcp_no_delay()`
    - `Config::set_timeout()` - per-request timeout.
    - `Config::set_max_connections_per_host()`
    - `Config::set_tls_config()` - only available on `h1-client` or `h1-client-rustls`.
    - More config may be available from the underlying [`http_client::Config`](https://docs.rs/http-client/6/http_client/struct.Config.html).
    - Easily turns into a `Client` via `std::convert::TryInto`.
- Extra `RequestBuilder` helpers for setting the body from different sources.
    - `body_json()`, `body_string()`, `body_bytes()`, `body_file()`.
- `Client::request()` for making arbitrary HTTP-method requests from a client.

### Improvements
- The `h1-client` backend now uses a shared client for 'one-off' style (`surf::get()`, etc) requests.
    - The `curl-client` and `hyper-client` backends already did this.
- The `wasm-client` feature now pulls in `getrandom`'s `"js"` feature.
    - This isn't a problem since the wasm client only works in a web/emscripten environment anyways.

### Deprecations
- `Client::set_base_url` has been deprecated in favor of `Config`.

### Docs
- Several docs fixes
- Minor 'branding' changes

## [2.2.0] - 2021-03-02

If you use the `h1-client`, upgrading to this release is recommended.

### Additions
- `h1-client-rustls` feature flag, for using the [`async-h1`](https://github.com/http-rs/async-h1) client with [`rustls`](https://github.com/ctz/rustls) as the TLS backend.
    - The TLS backend for `h1-client` is still `async-native-tls`.
- Per-request middleware, provided by `RequestBuilder::middleware(&mut self, impl Middleware)`.
- `AsRef<Headers>` and `AsMut<Headers>` for `surf::Request` and `surf::Response`.

### Fixes
- Relative redirects should now be handled by the `RedirectMiddleware`.
- The `h1-client` feature should now properly work with `http-client` 6.3.0+ without additional feature specification.

### Docs
- The `http` docs now link to the live, up-to-date `http_types` docs.

### Internal
- Various CI improvements.

## [2.1.0] - 2020-10-23

This minor release contains follow-up fixes and improvements to Surf 2.0.

### Additions
- Added a `hyper-client` cargo feature for enabeling a [hyper][] client backend via [http-client][].

### Fixes
- Fixed `base_url` not being propagated to the `Client` instance in middleware.

### Documentation
- Updated documentation for `set_base_url()`.

[http-client]: https://crates.io/crates/http-client
[hyper]: https://crates.io/crates/hyper

## [2.0.0] - 2020-10-05

[Docs](https://docs.rs/surf/2.0.0)

This major release of Surf contains substantial improvements through a variety of changes and additions.

_(Note: this is a cumulative list of changes since Surf 1.0.3)_

Notable mentions include:
- Uses stable standard library [`futures`][]!
- Much more type compatibility with [Tide][] via [http-types][]!
- Re-usable `Client` which is able to make use of connection pooling [under the hood](https://github.com/http-rs/http-client).
- Reduced generics for `Client` and `Middleware`.
- Re-worked `Middleware` to use [`async_trait`][].

### Major Changes
Surf 2 contains some large API changes, as noted here:

#### http-types

Surf has switched the common backing type interface (`surf::http`) from the [http][] (`hyperium/http`) crate to [http-types][], which covers a larger set of HTTP-related functionality than `hyperium/http` does, and allows Surf to use the [url standard][].

This affects any type that came from `surf::http`, such as `StatusCode` ([old][StatusCode http]|[new][StatusCode http-types]), and includes some new patterns, such as [`Body`][].

For more information, see [this blog post](https://blog.yoshuawuyts.com/async-http/#shared-abstractions).

#### Errors

`surf::Exception`, which was a plain `Box<dyn Error + Send + Sync + 'static>`, is no more.

Surf now exports a structured [`surf::Error`][] type, which holds a `StatusCode` alongside a dynamic error object.
Just like [`anyhow`][], any error can be cast to this type using the `?` operator.

For more information, see [this blog post](https://blog.yoshuawuyts.com/async-http/#error-handling).

#### Middleware

**New middleware:**
```rust
use surf::middleware::{Middleware, Next};
use surf::{Client, Request, Response, Result};

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

#### RequestBuilder

The top-level convenience request methods, `surf::get()`, `surf::post()`, etc, now return a [`RequestBuilder`][] rather than a [`Request`][] directly.
Most `RequestBuilder` functions have shorthand names: `RequestBuilder::body()` compared to `Request::set_body()`.

```rust
let res = surf::get("http://example.org") // Now returns a `surf::RequestBuilder`!
    .header(a_header, a_value)
    .body(a_body)
    .await?;
```

Overall usage was kept the same where possible and reasonable, however now a `surf::Client` must be used when using middleware.

```rust
let client = surf::client()
    .with(some_middleware);

let res = client::post(url)
    .header(a_header, a_value)
    .body(a_body)
    .await?;
```

Alternately:
```rust
let client = surf::client()
    .with(some_middleware);

let req = surf::post(url)
    .header(a_header, a_value)
    .body(a_body);
let res = client.send(req).await?;
```

#### Mime

Surf has switched from the [`mime`][] crate to [`surf::http::Mime`][] from [http-types][].

For more information, see [this blog post](https://blog.yoshuawuyts.com/async-http/#shared-abstractions).

### Additions
- Switched from [`hyperium/http`][http] to [http-types][].
- `surf::Request` added many methods that exist in `tide::Request`.
- `surf::Response` added many methods that exist in `tide::Response`.
- `surf::http`, an export of `http_types`, similar to `tide::http`.
- `surf::middleware::Redirect`, a middleware to handle redirect status codes.
- All conversions for `Request` and `Response` between `http_types` and `surf` now exist.
- `http_types::{Error, Result}` are re-exported as `surf::{Error, Result}`.
- A new `h1-client` feature enables the new [async-h1] backend.
- Transcode responses from non-UTF8 charsets using the on-by-default `encoding` feature.

### Removals
- Removed `native-client` feature flag in favor of direct `curl-client` default.
- Removed `hyper-client` feature flag. (Pending re-addition, see: [#234][])
- Removed `Request::body_string()`, `Request::body_json()`, etc.
  - This functionality is now done via [`Body`], or `Client::recv_json()`, `RequestBuilder::recv_json()`, etc.

### Changes
- Updated to use stable [`futures`].
- `wasm-client` feature is no longer automatic and must be set via cargo features.
- All client feature flags are now mutually exclusive. `curl-client` is the default.
- `surf::method_name` "one-off" methods now use a shared client internally if the client is `curl-client`. (Default)
- `Client::with_http_client()` is now generic for any `HttpClient` rather than taking an `Arc<dyn HttpClient>`.
  - (The http client is still stored internally as a dynamic pointer.)
- `HttpClient` has been upgraded to 6.0, removing `Clone` from the built in client backends.
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
- `Client` is now built on top of [`http-client`](https://github.com/http-rs/http-client).
- `surf::url` has been moved to `surf::http::url`, with a `surf::Url` shortcut.

### Internal
- Reduce copies when parsing URLs.
- Now only depends on `futures_util` rather than all of `futures`.
- `wasm-client` now has proper headless browser CI testing.
- Use Clippy in CI.
- Set up an MSRV in CI.
- Stop hitting the network when running tests.

### Changes since 2.0.0-alpha.7
- Added: `RequestBuilder` now has a `.query()` function for setting structured querystrings.
- Changed: `Client::send()` and `RequestBuilder::send()` are now plain async functions and no longer return [`BoxFuture`][]s.
- Changed: `surf::url` has been moved to `surf::http::url`, with a `surf::Url` shortcut.

[#234]: https://github.com/http-rs/surf/pull/234
[`anyhow`]: https://crates.io/crates/anyhow
[`async_trait`]: https://docs.rs/async-trait/0.1.41/async_trait/
[`futures`]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
[`mime`]: https://docs.rs/mime/0.3.14/mime/index.html
[`surf::http::Mime`]: https://docs.rs/surf/2.0.0/surf/http/struct.Mime.html
[`surf::Error`]: https://docs.rs/surf/2.0.0/surf/struct.Error.html
[`Body`]: https://docs.rs/surf/2.0.0/surf/struct.Body.html
[`BoxFuture`]: https://docs.rs/futures-util/0.3.5/futures_util/future/type.BoxFuture.html
[`Request`]: https://docs.rs/surf/2.0.0/surf/struct.Request.html
[`RequestBuilder`]: https://docs.rs/surf/2.0.0/surf/struct.RequestBuilder.html
[async-h1]: https://crates.io/crates/async-h1
[http]: https://docs.rs/http/0.2.1/http
[http-types]: https://github.com/http-rs/http-types
[url standard]: https://crates.io/crates/url
[StatusCode http]: https://docs.rs/http/0.2.1/http/status/struct.StatusCode.html
[StatusCode http-types]: https://docs.rs/http-types/2.5.0/http_types/enum.StatusCode.html
[Tide]: https://github.com/http-rs/tide

## [2.0.0-alpha.7] - 2020-09-29

### Fixes
- Downgrade rust_2018_idioms from forbid to warn for compilation with newer deps.


## [2.0.0-alpha.6] - 2020-09-27

This is an alpha release in preparation of 2.0.0, so you can start using Surf with stable futures. The aim is for this to be the last 2.0 alpha release.

As of this release, `surf::get()`, `surf::post()`, etc, now use a globally shared client internally, allowing for easier access to optimizations such as connection pooling.

### Removals
- Removed `native-client` feature flag in favor of direct `curl-client` default.

### Changes
- `wasm-client` feature is no longer automatic and must be set via cargo features.
- All client feature flags are now mutually exclusive. `curl-client` is the default.
- `surf::method_name` "one-off" methods now use a shared client internally if the client is `curl-client`. (Default)
- `Client::with_http_client()` is now generic for any `HttpClient` rather than taking an `Arc<dyn HttpClient>`.
  - (The http client is still stored internally as a dynamic pointer.)
- `HttpClient` has been upgraded to 6.0, removing `Clone` from the built in client backends.

### Fixes
- Surf can once again build with `--no-default-features` (and no client).
- Doc updates

### Internal
- `wasm-client` now has proper headless browser CI testing.

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

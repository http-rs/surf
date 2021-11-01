<h1 align="center">Surf</h1>
<div align="center">
 <strong>
   Surf the web
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/surf">
    <img src="https://img.shields.io/crates/v/surf.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/surf">
    <img src="https://img.shields.io/crates/d/surf.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/surf">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/surf">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/http-rs/surf/blob/main/.github/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://discord.com/invite/x2gKzst">
      Chat
    </a>
  </h3>
</div>

<div align="center">
  <sub>Built with 🌊 by <a href="https://github.com/http-rs">The http-rs team</a>
</div>

<br/>

### Surf the web - HTTP client framework

Surf is a Rust HTTP client built for ease-of-use and multi-HTTP-backend flexibility.
Whether it's a quick script, or a cross-platform SDK, Surf will make it work.

- Extensible through a powerful middleware system
- Multiple HTTP back-ends that can be chosen
- Reuses connections through a configurable `Client` interface
- Fully streaming requests and responses
- TLS enabled by default (native tls or rustls)
- Built on async-std (with optional tokio support)

## Examples
```rust
let mut res = surf::get("https://httpbin.org/get").await?;
dbg!(res.body_string().await?);
```

It's also possible to skip the intermediate `Response`, and access the response type directly.
```rust
dbg!(surf::get("https://httpbin.org/get").recv_string().await?);
```

Both sending and receiving JSON is real easy too.
```rust
#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

let uri = "https://httpbin.org/post";
let data = &Ip { ip: "129.0.0.1".into() };
let res = surf::post(uri).body_json(data)?.await?;
assert_eq!(res.status(), 200);

let uri = "https://api.ipify.org?format=json";
let Ip { ip } = surf::get(uri).recv_json().await?;
assert!(ip.len() > 10);
```

And even creating streaming proxies is no trouble at all.

```rust
let req = surf::get("https://img.fyi/q6YvNqP").await?;
let body = surf::http::Body::from_reader(req, None);
let res = surf::post("https://box.rs/upload").body(body).await?;
```

Setting configuration on a client is also straightforward.

```rust
use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config};
use surf::Url;

let client: Client = Config::new()
    .set_base_url(Url::parse("http://example.org")?)
    .set_timeout(Some(Duration::from_secs(5)))
    .try_into()?;

let mut res = client.get("/").await?;
println!("{}", res.body_string().await?);
```

## Features
The following features are available. The default features are
`curl-client`, `middleware-logger`, and `encoding`
- __`curl-client` (default):__ use `curl` (through `isahc`) as the HTTP backend.
- __`h1-client`:__ use `async-h1` as the HTTP backend with native TLS for HTTPS.
- __`h1-client-rustls`:__ use `async-h1` as the HTTP backend with `rustls` for HTTPS.
- __`hyper-client`:__ use `hyper` (hyper.rs) as the HTTP backend.
- __`wasm-client`:__ use `window.fetch` as the HTTP backend.
- __`middleware-logger` (default):__ enables logging requests and responses using a middleware.
- __`encoding` (default):__ enables support for body encodings other than utf-8.

## Installation

Install OpenSSL - 
- Ubuntu - ``` sudo apt install libssl-dev ```
- Fedora - ``` sudo dnf install openssl-devel ```

Make sure your rust is up to date using: 
``` rustup update ```

With [cargo add](https://github.com/killercup/cargo-edit#Installation) installed :
```sh
$ cargo add surf
```

## Safety

This crate makes use of a single instance of `unsafe` in order to make the WASM
backend work despite the `Send` bounds. This is safe because WASM targets
currently have no access to threads. Once they do we'll be able to drop this
implementation, and use a parked thread instead and move to full multi-threading
in the process too.

## Contributing

Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## See Also

- [http-rs/http-client](https://github.com/http-rs/http-client)
- [http-rs/http-service](https://github.com/http-rs/http-service)
- [http-rs/tide](https://github.com/http-rs/tide)

## Thanks

Special thanks to [prasannavl](https://github.com/prasannavl) for donating the
crate name, and [sagebind](https://github.com/sagebind) for creating an easy to
use `async` curl client that saved us countless hours.

## License

[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/surf.svg?style=flat-square
[2]: https://crates.io/crates/surf
[3]: https://img.shields.io/travis/http-rs/surf/main.svg?style=flat-square
[4]: https://travis-ci.org/http-rs/surf
[5]: https://img.shields.io/crates/d/surf.svg?style=flat-square
[6]: https://crates.io/crates/surf
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/surf
[releases]: https://github.com/http-rs/surf/releases
[contributing]: https://github.com/http-rs/surf/blob/main/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/http-rs/surf/labels/good%20first%20issue
[help-wanted]: https://github.com/http-rs/surf/labels/help%20wanted

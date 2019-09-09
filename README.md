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
  <!-- Build Status -->
  <a href="https://travis-ci.com/rustasync/surf">
    <img src="https://travis-ci.com/rustasync/surf.svg?branch=master"
      alt="Build Status" />
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
    <a href="https://github.com/rustasync/surf/blob/master/.github/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/474974025454452766">
      Chat
    </a>
  </h3>
</div>

<div align="center">
  <sub>Built with 🌊 by <a href="https://github.com/rustasync">The Rust Async Ecosystem WG</a>
</div>

<br/>

Surf is a friendly HTTP client built for casual Rustaceans and veterans alike.
It's completely modular, and built directly for `async/await`. Whether it's a
quick script, or a cross-platform SDK, Surf will make it work.

- Multi-platform out of the box
- Extensible through a powerful middleware system
- Reuses connections through the `Client` interface
- Fully streaming requests and responses
- TLS/SSL enabled by default
- Swappable HTTP backends
- HTTP/2 enabled by default

## Examples

```rust
#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut res = surf::get("https://httpbin.org/get").await?;
    dbg!(res.body_string().await?);
    Ok(()) 
}
```

It's also possible to skip the intermediate `Response`, and access the response
type directly.

```rust
#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dbg!(surf::get("https://httpbin.org/get").recv_string().await?);
    Ok(()) 
}
```

Both sending and receiving JSON is real easy too.

```rust
use serde::{Deserialize, Serialize};
#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
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
    Ok(())
}
```

And even creating streaming proxies is no trouble at all.

```rust
#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let reader = surf::get("https://img.fyi/q6YvNqP").await?;
    let res = surf::post("https://box.rs/upload").body(reader).await?;
    Ok(())
}
```

## Installation
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
- [rustasync/http-client](https://github.com/rustasync/http-client)
- [rustasync/http-service](https://github.com/rustasync/http-service)
- [rustasync/tide](https://github.com/rustasync/tide)

## Thanks
Special thanks to [prasannavl](https://github.com/prasannavl) for donating the
crate name, and [sagebind](https://github.com/sagebind) for creating an easy to
use `async` curl client that saved us countless hours.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/surf.svg?style=flat-square
[2]: https://crates.io/crates/surf
[3]: https://img.shields.io/travis/rustasync/surf/master.svg?style=flat-square
[4]: https://travis-ci.org/rustasync/surf
[5]: https://img.shields.io/crates/d/surf.svg?style=flat-square
[6]: https://crates.io/crates/surf
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/surf

[releases]: https://github.com/rustasync/surf/releases
[contributing]: https://github.com/rustasync/surf/blob/master/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/surf/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/surf/labels/help%20wanted

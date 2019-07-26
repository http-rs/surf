<h1 align="center">Surf</h1>
<div align="center">
 <strong>
   Surf the web
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/tide">
    <img src="https://img.shields.io/crates/v/surf.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Build Status -->
  <a href="https://travis-ci.org/rustasync/surf">
    <img src="https://img.shields.io/travis/rustasync/surf.svg?style=flat-square"
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

## About

Surf is the Rust HTTP client we've always wanted. It's completely modular, and
built directly for `async/await`. Whether it's a quick script, or a
cross-platform SDK, Surf will make it work.

- Multi-platform out of the box
- Extensible through a powerful middleware system
- Reuses connections through the `Client` interface
- Fully streaming requests and responses
- TLS/SSL enabled by default
- Swappable HTTP backends (`hyper (default)`, `libcurl (wip)`, `fetch (wip)`)

## Examples
```rust,no_run
let mut res = surf::get("https://google.com").await?;
dbg!(res.body_string().await?);
```

## Installation
```sh
$ cargo add surf
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## See Also
- [rustasync/http-client](https://github.com/rustasync/http-client)
- [rustasync/http-service](https://github.com/rustasync/http-service)
- [rustasync/tide](https://github.com/rustasync/tide)

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
[contributing]: https://github.com/rustasync/surf/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/surf/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/surf/labels/help%20wanted

<div align="center">
  <h1>Fast CLI</h1>
  <p>
    <b>
      fast.com, but from the terminal
    </b>
  </p>
  <sub>
    Inspired by
    <a href="https://github.com/sindresorhus/fast-cli" target="_blank">fast-cli by Sindre Sorhus</a>.
  </sub>
</div>

## Abstract

As for [Captain Hook](https://github.com/fdionisi/fast-cli), also
this repository is inspired by a well-known
[Node.js](https://github.com/nodejs/node) project:
[fast-cli](https://github.com/sindresorhus/fast-cli) by Sindre Sorhus. This
implementation aims to provide a smaller executable than the JavaScript
counterpart and to avoid the rigid requirement of Node.js. It's also an
opportunity to exercise my Rust and test [Zed](https://zed.dev) editor.

## Usage

```sh
fast
```

## Quick Install

> **Note** to install `fast-cli` you have to use `cargo install`. Unfortunately
> some dependencies requires openssl that
> [was removed](https://github.com/cross-rs/cross/issues/229) from `cross-rs`.
>
> I'd be happy to provide cross-platform executables, but it's not on top of my
> priority list. Contributions are most welcome.

```sh
cargo install fast-cli
```

To update the Captain Hook with Cargo, remember to force re-installing the
binary.

```sh
cargo install -f fast-cli
```

It is only possible to install `fast-cli` by building from source:
```sh
git clone git@github.com:fdionisi/fast-cli.git
cd fdionisi/fast-cli
cargo install --path crates/cli
```

## License

_Fast CLI_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

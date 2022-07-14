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

It is possible to install `fast-cli` in two flavours:

- With Shell:
  ```sh
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/fdionisi/fast-cli/main/scripts/install)"
  ```

  To update the Fast CLI itself, rerun the above script. It will replace the
  current version with the latest one.

- With Cargo:
  ```sh
  cargo install fast-cli
  ```

  To update the Fast CLI with Cargo, remember to force re-installing the
  binary.

  ```sh
  cargo install -f fast-cli
  ```

## License

_Fast CLI_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

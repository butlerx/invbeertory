# Welcome to invbeertory 👋

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
[![Documentation](https://img.shields.io/badge/documentation-yes-brightgreen.svg)](https://github.com/butlerx/invbeertory#readme)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/butlerx/invbeertory/graphs/commit-activity)
[![License: MIT](https://img.shields.io/github/license/butlerx/invbeertory)](https://github.com/butlerx/invbeertory/blob/master/LICENSE)

> Simple beer inventory display

### ✨ [Demo, pints.me](https://pints.me)

## Install

If you don't already have it installed, it's time to install Rust:
<https://www.rust-lang.org/tools/install>. The rest of this guide assumes a
typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target
installed. If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show:
[Trunk]. Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host
it.

### Release

```bash
trunk build --release
```

## Author

👤 **Cian Butler <butlerx@notthe.cloud>**

- Website: [cianbutler.ie](https://cianbutler.ie)
- Mastodon: [@butlerx@mastodon.ie](https://mastodon.ie/@butlerx)
- Github: [@butlerx](https://github.com/butlerx)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/butlerx/invbeertory/issues).

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2019
[Cian Butler <butlerx@notthe.cloud>](https://github.com/butlerx).

This project is
[MIT](https://github.com/butlerx/invbeertory/blob/master/LICENSE) licensed.

---

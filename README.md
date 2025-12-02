# Coyote-rs

Create `HTML` documents in Rust.

HTML in, HTML out. No suprises, no dependencies.

[![Tests](https://github.com/w-lfpup/coyote-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/w-lfpup/coyote-rs/actions/workflows/tests.yml)

## Install

`Coyote-rs` is available on [crates.io](https://crates.io/crates/coyotes) as `coyotes`:

```sh
cargo add coyotes
```

Or add directly from git:

```sh
cargo add --git https://github.com/w-lfpup/coyote-rs
```

## Components

Create document fragments with coyote [components](./components.md).

```rust
use coyotes::{Component, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Document builders

Render `html` with [document builders](./document_builders.md).

```rust
use coyotes::Html;

let html = Html::new();

if let Ok(document) = html.render(&hello_world()) {
    println!("{}", document);
};
```

The output will be:
```html
<p>hai :3</p>
```

## Spaces

`Coyote` never spaces or new lines.

Learn more about how `coyote` handles [spacing](./spacing.md).

## License

`Coyote-rs` is released under the BSD 3-Clause License.

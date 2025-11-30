# Coyote-rs

Create `HTML` documents with function components in Rust.

There are no dependencies. There are no macros.

HTML in, HTML out. No suprises and very little overhead.

## Install

`Coyote-rs` is available on [crates.io](https://crates.io) as `koyote`:

```sh
cargo install coyoteh
```

Or install directly from git:

```sh
cargo install --git https://github.com/w-lfpup/coyote-rs
```

## Components

Create document fragments with coyote [components](./components.md).

```rust
use coyote_html::{Component, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Html

Render components as `html` with [document builders](./document_builders.md).

```rust
use coyote_html::Html;

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

Spacing is meaningful in html so `Coyote` respects the lines and spaces defined by developers.

Learn more about how `coyote` handles [spacing](./spacing.md).

## License

`Coyote-rs` is released under the BSD 3-Clause License.

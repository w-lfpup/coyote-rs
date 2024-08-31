
## Coyote Html

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote_html
```

### Hello, world!

The example below creates an html fragment from a coyote component function.

```rust
use coyote::{Component, tmpl};
use coyote_html::Html

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new()
    let document = html.compose(&hello_world);

    println!("{}", document);
}
```

But this is not very useful and not very pretty.

### Hello, pretty world!

The example below creates a formatted / opinionated html document from a string. 

This is more useful and also pretty.

```rust
use coyote::{Component, tmpl};
use coyote_html::{Html, Sieve, pretty_html}

fn article() -> Component {
    tmpl("
    <article>   <p>hai :3</p>
    </article>", [])
}

fn main() {
    let hello_article = article();

    let html = Html::new()
    let document = html.compose(&hello_article);

    println!("{}", pretty_html(document, Sieve::new());
}
```

And the output will be:
```html
<article>
    <p>hai :3</p>
</article>
```

### Hello, safe world!

Technically, any text can be injected into any template. Which presents a challenge! How can we trust our own templates without extensive testing?

The example below creates a _safer_ fragment for client-side renders from a string. 

```rust
use coyote_html::{ClientSieve, pretty_html};

fn main() {
    let hello_world = "
    <article><script>
        console.log(\"do something dangerous\");</script>
    <style>* { color: transparent }</style>
    <p>hai :3
    <p></article>";
    
    println!("{}", pretty_html(document, ClientSieve::new());
}
```

And the output will be:
```html
<article><p>hai :3</p></article>
```

This is very useful, pretty, and _safe_.

`Coyote Html` uses `sieves` to strip unwanted elements and components.

For instance the `ClietSieve` filters out elements with resposibilities outside of providing new document structure. So elements like `<script>` and `<style>` aren't just escaped, they're ripped from an html fragment entirely.

## License

`Coyote-rs` is released under the BSD-3-Clause License

# Document Builders

`Coyote` renders components with `document builders`.

## Html

### Hello, world!

The example below creates an html document from a component function.

```rust
use coyotex::{Component, Html, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}

fn main() {
    let html = Html::new();

    if let Ok(document) = html.render(&hello_world()) {
        println!("{}", document);
    }; 
}
```

The output will be:
```html
<p>hai :3</p>
```

## Html Only

### Hello, safer world!

The example below creates a _safer_ fragment for client-side renders using `HtmlOnly`.

This document builder removes all instances of `link`, `style`, and `script` elements.

```rust
use coyote_rs::{HtmlOnly, Component, tmpl};

fn hello_world() -> Component {
    tmpl(
        "<p>hai {} >:3</p>",
        [malicious()],
    )
}

fn malicious() -> Component {
    tmpl("
        <link rel=stylesheet href=malicious_stylesheet.css>
        <style>
            * { color: malicious-blue; }
        </style>
        <script>
            console.log('malicious rawrr!');
        </script>
    ", [])
}

fn main() {    
    let html_only = HtmlOnly::new();    
    
    if let Ok(document) = html_only.render(&hello_world()) {
        println!("{}", document);
    }; 
}
```

The output will be:
```html
<p>hai >:3</p>
```

## Errors

A document builder will return an error when:
- a template is unbalanced
- an attribute contains a forbidden glyph
- a render exceeds a memory limit

### Forbidden attribute glyphs

The following characters are forbidden in html attributes:
- <
- =
- "
- \
- /
- \>
- {

Coyote will return an `error` when any of the characters above are found in an attribute component.

### Unbalanced templates

Balanced templates are templates without unclosed tags.

Coyote will return an `error` when a template is unbalanced.

The following template is balanced:

```rust
tmpl("<p></p>", [])
```

The following template is also balanced because the `input` element is a [void element](https://developer.mozilla.org/en-US/docs/Glossary/Void_element).

```rust
tmpl("<input>", [])
```

The following template is not balanced and will return an `error`:

```rust
tmpl("<span>", [])
```

### Document memory limits

Coyote will return an error when a document exceeds a predefined memory limit.

The fallback memory limit is `16mb` which is a sizable html document.

The following section demostrates how to customize document builder parameters like memory limits.

## Customize a document builder

Document builders can be custimized using a params object:

```rs
use coyote_rs::{Html, RendererParams};

let renderer_params = RendererParams {
    respect_indentation: true,
    cache_memory_limit: 32 * 1024 * 1024,
    document_memory_limit: 128 * 1024 * 1024,
};

let html = Html::from(&renderer_params);
let html_only = HtmlOnly::from(&renderer_params);
```

Memory limits are defined in bytes.

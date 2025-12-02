# Document Builders

`Coyote` renders components with `document builders`.

## Html

### Hello, world!

The example below creates an html document from a component function.

```rust
use coyotes::{Component, Html, tmpl};

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

The `HtmlOnly` document builder is meant for HOTW scenarios that cannot affort to render elements with side-effects.

Consider the following component with malicious intent:

```rust
use coyotes::{Component, tmpl};

fn malicious() -> Component {
    tmpl("
        <link rel=stylesheet href=malicious_stylesheet.css>
        <style>
            * { color: malicious-blue; }
        </style>
        <script>
            console.log('malicious rawrr!');
        </script>
        ",
        []
    )
}

fn hello_world() -> Component {
    tmpl(
        "<p>hai {} :3</p>",
        [malicious()],
    )
}
```

The malicious component could load elements that can mutate the DOM and  break your site.

The `HtmlOnly` document builder removes all instances of `link`, `style`, and `script` elements.

```rs
use coyotes::{HtmlOnly};

fn main() {
    let html_only = HtmlOnly::new();    
    
    if let Ok(document) = html_only.render(&hello_world()) {
        println!("{}", document);
    }; 
}
```

So the `hello_world` components above renders without the mutative components:

```html
<p>hai :3</p>
```

## Errors

A document builder will return an error when:
- a template is unbalanced
- an attribute contains a forbidden glyph
- a render exceeds a memory limit

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

The following template does not close all tags. So it is not balanced and document builders will return an `error`:

```rust
tmpl("<span>", [])
```

### Forbidden attribute glyphs

The following characters are [forbidden](https://html.spec.whatwg.org/multipage/syntax.html#attributes-2) in html attributes:
`< > = " \ /`

The bracket-character `{` is forbidden in attribute components by `coyote`.

Coyote will return an `error` when any forbidden characters are found in an attribute component.

### Document memory limits

Coyote will return an error when a document exceeds a predefined memory limit.

The fallback memory limit is `16mb` which is a sizable document.

The following section demostrates how to customize document builder parameters like memory limits.

## Customize a document builder

Document builders can be custimized using a params object:

```rs
use coyotes::{Html, DocumentParams};

let params = DocumentParams {
    cache_memory_limit: 32 * 1024 * 1024,
    document_memory_limit: 128 * 1024 * 1024,
    embedded_content: String::from("svg"),
    respect_indentation: true,
};

let html = Html::from(&params);
let html_only = HtmlOnly::from(&params);
```

Memory limits are defined in bytes.

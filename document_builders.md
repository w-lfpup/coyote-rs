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

Its output will be:
```html
<p>hai :3</p>
```

## Html Only

### Hello, safer world!

The `HtmlOnly` document builder is meant for html-over-the-wire (HOTW). Scenarios that cannot afford side-effects benefit from this builder.

Consider the following component with a malicious intent:

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

The malicious component can potentially mutate the DOM and break your site.


But when the `hello_world` component above is rendered with the `HtmlOnly` document builder:

```rs
use coyotes::{HtmlOnly};

fn main() {
    let html_only = HtmlOnly::new();    
    
    if let Ok(document) = html_only.render(&hello_world()) {
        println!("{}", document);
    }; 
}
```

The builder removes all instances of `link`, `style`, and `script` elements:

```html
<p>hai :3</p>
```

## Errors

A document builder returns an error when:
- a template is unbalanced
- an attribute contains a forbidden glyph
- a render exceeds a memory limit

### Unbalanced templates

Balanced templates are templates without unclosed tags.

Coyote returns an `error` when a template is _unbalanced_.

#### Examples of balance
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

Coyote returns an `error` when any forbidden characters are found in an attribute component.

### Document memory limits

Coyote returns an error when a document exceeds a predefined memory limit.

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

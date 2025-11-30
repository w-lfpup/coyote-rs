# Document Builders

`Coyote` renders components with `document builders`.

## Html

### Hello, world!

The example below creates an html document from a coyote component function.

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

This document builder avoids all instances of `link`, `style`, and `script` elements.

```rust
use coyoteh::{HtmlOnly, Component, tmpl};

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

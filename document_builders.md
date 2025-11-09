# Document Builders

`Coyote` renders templates with `rulesets` and `document builders`.

## Html

### Hello, world!

The example below creates an html document from a coyote component function.

```rust
use coyote::{Component, Html, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}

fn main() {
    let html = Html::new();

    if let Ok(document) = html.build(&hello_world()) {
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

```rust
use coyote::{HtmlOnly, Component, tmpl};

fn malicious_component() -> Component {
    tmpl("
        <link rel=stylesheet href=malicious_stylesheet.css>
        <style>
            * { color: malicious-blue; }
        </style>
        <script>
            console.log('malicious! rawr!');
        </script>
    ", [])
}

fn hello_world() -> Component {
    tmpl(
        "{}<p>hai >:3</p>",
        [malicious_component()],
    )
}

fn main() {
    let hello_world = hello_world();
    
    let client_html = HtmlOnly::new();    
    
    if let Ok(document) = client_html.build(&hello_world) {
        println!("{}", document);
    }; 
}
```

The output will be:
```html
<p>hai >:3</p>
```

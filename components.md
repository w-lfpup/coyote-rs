# Components

`Coyote` creates documents with components.

## Components as an IMR

Components are not HTML or XML.

Components are an (I)ntermediate (R)endering (F)ormat.

They are the _potential_ for HTML or XML.

## The template component

## Syntax story

`Coyote` templates support self-closing tags, void elements, and jsx-like fragments:

```rs
tmpl(
    "
    <article>
        <>
            <p>no waaaay?</p>
            <custom-element />
            <menuitem>yoooo</menuitem>
            <input type=button value='high-five!' />
        </>
    </article>
    ",
    []
)
```

However, `coyote` will only render the HTML5 standard:

```html
<article>
    <p>no waaaay?</p>
    <custom-element></custom-element>
    <input value=button value="high-five!">
</article>
```

The goal is to provide a robust and flexible template syntax while adhering modern standards.

## Function Components

Function components are functions that return components!

```rust
use coyotes::{Component, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Tetmplate injections

Template `injections` nest templates and assign attributes.

There are only two valid _injections_ in a `tmpl` component:
- attribute injections
- descendant injections

Likewise there are only two valid injection locations in a `tmpl` component:

```rs
fn injection_story() -> Component {
    let attribute = attr("uwu");
    let descendant = text("hai :3")

    tmpl(
        "
        <article {}>
            {}
        </article>
        ",
        [attribute, descendant]
    )
}
```

Which renders:

```html
<article uwu>
    hai :3
<article>
```

Any other instance of brackets `{}` in a template component will not register as an injection.

### Escape the left-bracket `{` character

To use a left-bracket `{` in a template without creating an injection, use the html escape charactor `&123;`.

So the following template:

```rust
tmpl("hellooo, &#123; world }", []); 
```

Will render a left-bracket as text:

```html
helloooo { world }
```

## Lists of components

The `list` and `vlist` components immitate the `node -> [node, text, node, ...]` heiarchy of an xml-like document.

The example below creates a form defined by lists of attributes, templates, and text.

```rust
use coyotes::{Component, attr_val, list, text, tmpl};

fn submit_button() -> Component {
    tmpl("<input type=submit value='yus ^_^'>", [])
}

fn form() -> Component {
    let attributes = list([
        attr_val("action", "/uwu"),
        attr_val("method", "post"),
    ]);

    let mut descendants: Vec<Component> = Vec::new();
    descendants.push(text("you're a good dog aren't you >:3"));
    descendants.push(submit_button());
    
    tmpl(
        "
        <form {}>
            {}
        </form>
        ",
        [attributes, vlist(descendants)],
    )
}
```

And the output will be:

```html
<form action="/uwu" method="post">
    you're a good dog aren't you >:3
    <input type=submit value="yus ^_^">
</form>
```

## Types of components

`Components` are the atomic chunks used to build documents.

```rs
use coyotes::{
    attr,
    attr_val,
    text,
    tmpl,
    tmpl_string,
    list,
    vlist,
    Component::None,
}
```

#### Attribute

an element attribute

```rs
attr(name: &str)
```

#### Attribute with value

an attribute and value pair

```rs
attr_val(name: &str, value: &str)
```

#### Text

text with the HTML-safe escaped text

```rs
text(text_str: &str)
```

#### Template

a document fragment described by a static string template and a list of injections

```rs
tmpl(template_str: &'static str, injections: [Component; N])
```

#### Template string

A document fragment described by a string template and a list of injections. Useful for dynamically generated templates or templates found on disk / remote.

```rs
tmpl_string(template_str: &str, injections: [Component; N])
```

#### List

a list of components

```rs
list(components: [Component; N]) -> Component
```

#### Vector list

a vector list of components

```rs
vlist(components: Vec<Component>)-> Component
```

#### None

the abscence of a component

```rs
Component::None
```

## Document builders

Coyote renders components with [document builders](./document_builders.md).
